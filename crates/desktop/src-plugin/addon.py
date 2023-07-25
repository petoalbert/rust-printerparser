import bpy
import requests
import os
import json
from threading import Timer
from urllib.parse import quote_plus

bl_info = {
    "name": "Timeline Tool",
    "blender": (2, 80, 0),
    "category": "Object",
}

URL = 'http://127.0.0.1:8080'


def get_file_path():
    return bpy.data.filepath


def get_db_path(file_path):
    return os.path.join(os.path.dirname(file_path), ".timeline")


def call_commit_api(message):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/commit'
    headers = {'Content-Type': 'application/json'}
    data = {
        'db_path': db_path,
        'file_path': file_path,
        'message': message,
    }

    return requests.post(url, headers=headers, data=json.dumps(data))


def call_checkpoints_api(current_branch):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/checkpoints/{quote_plus(db_path)}/{quote_plus(current_branch)}'

    response = requests.get(url)
    return response.json()


def call_restore_api(hash):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/restore'
    headers = {'Content-Type': 'application/json'}
    data = {
        'db_path': db_path,
        'file_path': file_path,
        'hash': hash,
    }

    return requests.post(url, headers=headers, data=json.dumps(data))


def call_new_branch_api(new_branch_name):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/branches/new'
    headers = {'Content-Type': 'application/json'}
    data = {
        'db_path': db_path,
        'file_path': file_path,
        'branch_name': new_branch_name,
    }

    return requests.post(url, headers=headers, data=json.dumps(data))


def call_list_branches_api():
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/branches/{quote_plus(db_path)}'

    response = requests.get(url)
    return response.json()


def call_get_current_branch_api():
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/branches/current/{quote_plus(db_path)}'

    response = requests.get(url)
    return response.json()


def call_switch_branch_api(new_branch_name):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/branches/switch'
    headers = {'Content-Type': 'application/json'}
    data = {
        'db_path': db_path,
        'file_path': file_path,
        'branch_name': new_branch_name,
    }

    return requests.post(url, headers=headers, data=json.dumps(data))


def call_list_branches_operator():
    bpy.ops.wm.list_branches_operator()


def call_list_checkpoints_operator():
    bpy.ops.wm.list_checkpoints_operator()


def call_get_current_branch_operator():
    bpy.ops.wm.get_current_branch_operator()


def run_onload_ops():
    call_get_current_branch_operator()
    call_list_branches_operator()
    call_list_checkpoints_operator()


def save_file():
    bpy.ops.file.pack_all()
    bpy.ops.wm.save_mainfile()


def refresh_file():
    bpy.ops.wm.revert_mainfile()


class ListCheckpointsOperator(bpy.types.Operator):
    """List checkpoints for this branch"""
    bl_idname = "wm.list_checkpoints_operator"
    bl_label = "List Checkpoints"

    def execute(self, context):
        current_branch = context.scene.current_branch if context.scene.current_branch != None else "main"
        items = call_checkpoints_api(current_branch)

        bpy.context.scene.checkpoint_items.clear()

        for i in items:
            item = bpy.context.scene.checkpoint_items.add()
            item.hash = i['hash']
            item.message = i['message']

        return {'FINISHED'}


class ListBranchesOperator(bpy.types.Operator):
    """List branches for this project"""
    bl_idname = "wm.list_branches_operator"
    bl_label = "List Branches"

    def execute(self, context):
        branches = call_list_branches_api()

        self.report({'INFO'}, f"{branches}")

        bpy.context.scene.branch_items.clear()

        for branch in branches:
            item = bpy.context.scene.branch_items.add()
            item.name = branch

        return {'FINISHED'}


class GetCurrentBranchOperator(bpy.types.Operator):
    """Find out which branch is active"""
    bl_idname = "wm.get_current_branch_operator"
    bl_label = "Get current branch"

    def execute(self, context):
        current_branch = call_get_current_branch_api()
        context.scene.current_branch = current_branch

        return {'FINISHED'}


class SwitchBranchesOperator(bpy.types.Operator):
    """Switch to another branch"""
    bl_idname = "wm.switch_branch_operator"
    bl_label = "Switch Branch"

    name: bpy.props.StringProperty(name="Branch name", default="")

    def execute(self, _):
        # TODO: if the file is unsaved, ask the user to confirm
        save_file()
        call_switch_branch_api(self.name)
        refresh_file()
        run_onload_ops()
        return {'FINISHED'}


class NewBranchOperator(bpy.types.Operator):
    """Create a new branch"""
    bl_idname = "wm.new_branch_operator"
    bl_label = "New Branch"

    name: bpy.props.StringProperty(name="Branch name", default="")

    def execute(self, _):
        call_new_branch_api(self.name)
        run_onload_ops()
        return {'FINISHED'}

    def invoke(self, context, event):
        return context.window_manager.invoke_props_dialog(self)


class CheckpointItem(bpy.types.PropertyGroup):
    message: bpy.props.StringProperty(name="Message", default="")
    hash: bpy.props.StringProperty(name="Hash", default="")


class BranchItem(bpy.types.PropertyGroup):
    name: bpy.props.StringProperty(name="Name", default="")


class RestoreOperator(bpy.types.Operator):
    """Restore a checkpoint"""
    bl_idname = "my.restore_operator"
    bl_label = "Restore"

    hash: bpy.props.StringProperty(name="Hash", default="")

    def execute(self, _):
        # TODO: if the file is unsaved, ask the user to confirm
        call_restore_api(self.hash)
        refresh_file()
        run_onload_ops()
        return {'FINISHED'}


class CreateCheckpointOperator(bpy.types.Operator):
    """Create a new checkpoint"""
    bl_idname = "my.create_checkpoint_operator"
    bl_label = "Create Checkpoint Operator"

    def execute(self, context):
        save_file()

        message = context.scene.commit_message
        response = call_commit_api(message)
        self.report({'INFO'}, f"{response.status_code}")
        run_onload_ops()
        return {'FINISHED'}


class RefreshOperator(bpy.types.Operator):
    """Refresh the plugin UI"""
    bl_idname = "my.refresh"
    bl_label = "Refresh"

    def execute(self, _):
        run_onload_ops()
        return {'FINISHED'}


class TimelinePanel(bpy.types.Panel):
    bl_label = "Timeline"
    bl_idname = "TimelinePanel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'Timeline'

    def draw(self, context):
        layout = self.layout

        branches_box = layout.box()
        branches_box.label(text="Branches")

        current_branch_row = branches_box.row()
        current_branch_row.label(text="Current branch: ")
        current_branch_row.label(text=context.scene.current_branch)

        for item in context.scene.branch_items:
            row = branches_box.row()
            row.label(text=item.name)
            row.operator("wm.switch_branch_operator",
                         text="Switch to").name = item.name
        branches_box.operator("wm.new_branch_operator", text="New branch")

        restore_box = layout.box()
        restore_box.label(text="Restore checkpoint")
        for item in context.scene.checkpoint_items:
            row = restore_box.row()
            row.label(text=item.message)
            row.operator("my.restore_operator",
                         text="Restore").hash = item.hash

        checkpoint_box = layout.box()
        checkpoint_box.label(text="New Checkpoint")
        checkpoint_box.prop(context.scene, "commit_message", text="")
        checkpoint_box.operator(
            "my.create_checkpoint_operator", text="Create Checkpoint")

        layout.operator("my.refresh", text="Refresh")


def register():
    bpy.utils.register_class(ListCheckpointsOperator)
    bpy.utils.register_class(ListBranchesOperator)
    bpy.utils.register_class(SwitchBranchesOperator)
    bpy.utils.register_class(NewBranchOperator)
    bpy.utils.register_class(GetCurrentBranchOperator)
    bpy.utils.register_class(CheckpointItem)
    bpy.utils.register_class(BranchItem)
    bpy.utils.register_class(RestoreOperator)
    bpy.utils.register_class(CreateCheckpointOperator)
    bpy.utils.register_class(RefreshOperator)
    bpy.utils.register_class(TimelinePanel)

    bpy.types.Scene.checkpoint_items = bpy.props.CollectionProperty(
        type=CheckpointItem)
    bpy.types.Scene.branch_items = bpy.props.CollectionProperty(
        type=BranchItem)
    bpy.types.Scene.current_branch = bpy.props.StringProperty(name="")
    bpy.types.Scene.commit_message = bpy.props.StringProperty(
        name="", options={'TEXTEDIT_UPDATE'})

    Timer(1, run_onload_ops, ()).start()


def unregister():
    bpy.utils.unregister_class(ListCheckpointsOperator)
    bpy.utils.unregister_class(ListBranchesOperator)
    bpy.utils.unregister_class(SwitchBranchesOperator)
    bpy.utils.unregister_class(NewBranchOperator)
    bpy.utils.unregister_class(GetCurrentBranchOperator)
    bpy.utils.unregister_class(CheckpointItem)
    bpy.utils.unregister_class(BranchItem)
    bpy.utils.unregister_class(RestoreOperator)
    bpy.utils.unregister_class(CreateCheckpointOperator)
    bpy.utils.unregister_class(RefreshOperator)
    bpy.utils.unregister_class(TimelinePanel)

    del bpy.types.Scene.checkpoint_items
    del bpy.types.Scene.branch_items
    del bpy.types.Scene.commit_message
    del bpy.types.Scene.current_branch


if __name__ == "__main__":
    register()
