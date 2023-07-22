import bpy
from threading import Timer
import requests
from urllib.parse import quote_plus
import json
import os

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

def call_checkpoints_api():
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = f'{URL}/checkpoints/{quote_plus(db_path)}'

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

def run_onload_ops():
    bpy.ops.wm.list_checkpoints_operator()
    bpy.ops.wm.list_branches_operator()

class ListCheckpointsOperator(bpy.types.Operator):
    bl_idname = "wm.list_checkpoints_operator"
    bl_label = "List Checkpoints"
    
    def execute(self, _):
        self.report({'INFO'}, "Calling checkpoints api...")
        items = call_checkpoints_api()
        self.report({'INFO'}, f"{items}")
        
        bpy.context.scene.checkpoint_items.clear()

        for i in items:
            item = bpy.context.scene.checkpoint_items.add()
            item.hash = i['hash']
            item.message = i['message']
    
        return {'FINISHED'}

class ListBranchesOperator(bpy.types.Operator):
    bl_idname = "wm.list_branches_operator"
    bl_label = "List Branches"
    
    def execute(self, _):
        self.report({'INFO'}, "Calling branches api...")
        branches = call_list_branches_api()

        self.report({'INFO'}, f"{branches}")

        bpy.context.scene.branch_items.clear()

        for branch in branches:
            item = bpy.context.scene.branch_items.add()
            item.name = branch
    
        return {'FINISHED'}

class SwitchBranchesOperator(bpy.types.Operator):
    bl_idname = "wm.switch_branch_operator"
    bl_label = "Switch Branch"

    name: bpy.props.StringProperty(name="Branch name", default="")

    def execute(self, _):
        call_switch_branch_api(self.name)

class CommitItem(bpy.types.PropertyGroup):
    message: bpy.props.StringProperty(name="Message", default="")
    hash: bpy.props.StringProperty(name="Hash", default="")

class BranchItem(bpy.types.PropertyGroup):
    name: bpy.props.StringProperty(name="Name", default="")

class CheckoutItemOperator(bpy.types.Operator):
    bl_idname = "my.checkout_item"
    bl_label = "Restore"

    hash: bpy.props.StringProperty(name="Hash", default="")

    def execute(self, _):
        call_restore_api(self.hash)
        bpy.ops.wm.revert_mainfile()
        run_onload_ops()
        return {'FINISHED'}

class CommitOperator(bpy.types.Operator):
    bl_idname = "my.commit"
    bl_label = "Commit Operator"

    def execute(self, context):
        bpy.ops.wm.save_mainfile()
        
        message = context.scene.commit_message
        response = call_commit_api(message)
        self.report({'INFO'}, f"{response.status_code}")
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

        for item in context.scene.branch_items:
            row = layout.row()
            row.label(text=item.name)
            row.operator("wm.switch_branch_operator", text="Switch to").name = item.name

        for item in context.scene.checkpoint_items:
            row = layout.row()
            row.label(text=item.message)
            row.operator("my.checkout_item", text="Restore").hash = item.hash

        layout.prop(context.scene, "commit_message", text="")

        row = layout.row()
        row.operator("my.commit", text="Create Checkpoint")

def register():
    bpy.utils.register_class(ListCheckpointsOperator)
    bpy.utils.register_class(ListBranchesOperator)
    bpy.utils.register_class(SwitchBranchesOperator)
    bpy.utils.register_class(CommitItem)
    bpy.utils.register_class(BranchItem)
    bpy.utils.register_class(CheckoutItemOperator)
    bpy.utils.register_class(CommitOperator)
    bpy.utils.register_class(TimelinePanel)

    bpy.types.Scene.checkpoint_items = bpy.props.CollectionProperty(type=CommitItem)
    bpy.types.Scene.branch_items = bpy.props.CollectionProperty(type=BranchItem)
    bpy.types.Scene.commit_message = bpy.props.StringProperty(name="")

    Timer(1, run_onload_ops, ()).start()

def unregister():
    bpy.utils.unregister_class(ListCheckpointsOperator)
    bpy.utils.unregister_class(ListBranchesOperator)
    bpy.utils.unregister_class(SwitchBranchesOperator)
    bpy.utils.unregister_class(CommitItem)
    bpy.utils.unregister_class(BranchItem)
    bpy.utils.unregister_class(CheckoutItemOperator)
    bpy.utils.unregister_class(CommitOperator)
    bpy.utils.unregister_class(TimelinePanel)

    del bpy.types.Scene.checkpoint_items
    del bpy.types.Scene.branch_items
    del bpy.types.Scene.commit_message

if __name__ == "__main__":
    register()