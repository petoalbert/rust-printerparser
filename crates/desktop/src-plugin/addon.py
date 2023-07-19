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

def get_file_path():
    return bpy.data.filepath

def get_db_path(file_path):
    return os.path.join(os.path.dirname(file_path), ".timeline")

def call_commit_api(message):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = 'http://127.0.0.1:8080/commit'
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
    url = f'http://127.0.0.1:8080/checkpoints/{quote_plus(db_path)}'

    response = requests.get(url)
    return response.json()

def call_restore_api(hash):
    file_path = get_file_path()
    db_path = get_db_path(file_path)
    url = 'http://127.0.0.1:8080/restore'
    headers = {'Content-Type': 'application/json'} 
    data = {
        'db_path': db_path,
        'file_path': file_path,
        'hash': hash,
    }

    return requests.post(url, headers=headers, data=json.dumps(data))

def onload_operator():
    bpy.ops.wm.onload_operator()

class OnloadOperator(bpy.types.Operator):
    bl_idname = "wm.onload_operator"
    bl_label = "Test Operator"
    
    def execute(self, context):
        self.report({'INFO'}, "Calling checkpoints api...")
        items = call_checkpoints_api()
        self.report({'INFO'}, f"{items}")
        
        bpy.context.scene.checkpoint_items.clear()

        for i in items:
            item = bpy.context.scene.checkpoint_items.add()
            item.hash = i['hash']
            item.message = i['message']
    
        return {'FINISHED'}

class CommitItem(bpy.types.PropertyGroup):
    message: bpy.props.StringProperty(name="Message", default="")
    hash: bpy.props.StringProperty(name="Hash", default="")

class CheckoutItemOperator(bpy.types.Operator):
    bl_idname = "my.checkout_item"
    bl_label = "Restore"
    hash: bpy.props.StringProperty(name="Hash", default="")

    def execute(self, context):
        call_restore_api(self.hash)
        bpy.ops.wm.revert_mainfile()
        onload_operator()
        return {'FINISHED'}

class CommitOperator(bpy.types.Operator):
    bl_idname = "my.commit"
    bl_label = "Commit Operator"

    def execute(self, context):
        bpy.ops.wm.save_mainfile()
        
        message = context.scene.commit_message
        response = call_commit_api(message)
        self.report({'INFO'}, f"{response.status_code}")
        onload_operator()
        return {'FINISHED'}

class TimelinePanel(bpy.types.Panel):
    bl_label = "Timeline"
    bl_idname = "TimelinePanel"
    bl_space_type = 'VIEW_3D'
    bl_region_type = 'UI'
    bl_category = 'Timeline'

    def draw(self, context):
        layout = self.layout

        # Dropdown list
        layout.prop(context.scene, "my_enum_prop", text="")

        # List of items
        for item in context.scene.checkpoint_items:
            row = layout.row()
            row.label(text=item.message)
            row.operator("my.checkout_item", text="Restore").hash = item.hash

        # Text field
        layout.prop(context.scene, "commit_message", text="")

        # Blue button at the bottom
        row = layout.row()
        row.operator("my.commit", text="Create Checkpoint")

def register():
    bpy.utils.register_class(CommitItem)
    bpy.utils.register_class(CheckoutItemOperator)
    bpy.utils.register_class(CommitOperator)
    bpy.utils.register_class(TimelinePanel)
    bpy.utils.register_class(OnloadOperator)
    bpy.types.Scene.my_enum_prop = bpy.props.EnumProperty(
        items=[
            ("OPTION1", "Option 1", ""),
            ("OPTION2", "Option 2", ""),
            ("OPTION3", "Option 3", ""),
            ("OPTION4", "Option 4", ""),
        ]
    )
    bpy.types.Scene.commit_message = bpy.props.StringProperty(name="")
    bpy.types.Scene.checkpoint_items = bpy.props.CollectionProperty(type=CommitItem)

    Timer(1, onload_operator, ()).start()

def unregister():
    bpy.utils.unregister_class(TimelinePanel)
    bpy.utils.unregister_class(CommitItem)
    bpy.utils.unregister_class(CheckoutItemOperator)
    bpy.utils.unregister_class(CommitOperator)
    bpy.utils.unregister_class(OnloadOperator)
    del bpy.types.Scene.checkpoint_items
    del bpy.types.Scene.my_enum_prop
    del bpy.types.Scene.commit_message

if __name__ == "__main__":
    register()