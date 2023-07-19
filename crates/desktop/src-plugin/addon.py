import bpy
import time
from threading import Timer
import requests

bl_info = {
    "name": "Timeline Tool",
    "blender": (2, 80, 0),
    "category": "Object",
}

def onload_operator():
    bpy.ops.wm.onload_operator()

class OnloadOperator(bpy.types.Operator):
    bl_idname = "wm.test_operator"
    bl_label = "Test Operator"
    def execute(self, context):
        for i in range(5):
            item = bpy.context.scene.my_items.add()
            item.name = f"Item {i+1}"
    
        return {'FINISHED'}

class MY_PG_Item(bpy.types.PropertyGroup):
    name: bpy.props.StringProperty(name="Item Name", default="Item")

class MY_OT_UpdateItem(bpy.types.Operator):
    bl_idname = "my.update_item"
    bl_label = "Update Item"
    item_index: bpy.props.IntProperty()

    def execute(self, context):
        context.scene.my_items[self.item_index].name = context.scene.my_string_prop
        return {'FINISHED'}

class MY_OT_Operator(bpy.types.Operator):
    bl_idname = "my.operator"
    bl_label = "My Operator"

    def execute(self, context):
        resp = requests.get('http://127.0.0.1:8080')
        self.report({'INFO'}, resp.text)
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
        for i, item in enumerate(context.scene.my_items):
            row = layout.row()
            row.label(text=item.name)
            row.operator("my.update_item", text="Update").item_index = i

        # Text field
        layout.prop(context.scene, "my_string_prop", text="")

        # Blue button at the bottom
        row = layout.row()
        row.operator("my.operator", text="Blue Button")

def register():
    bpy.utils.register_class(MY_PG_Item)
    bpy.utils.register_class(MY_OT_UpdateItem)
    bpy.utils.register_class(MY_OT_Operator)
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
    bpy.types.Scene.my_string_prop = bpy.props.StringProperty(name="")
    bpy.types.Scene.my_items = bpy.props.CollectionProperty(type=MY_PG_Item)

    Timer(1, onload_operator, ()).start()

def unregister():
    bpy.utils.unregister_class(TimelinePanel)
    bpy.utils.unregister_class(MY_PG_Item)
    bpy.utils.unregister_class(MY_OT_UpdateItem)
    bpy.utils.unregister_class(MY_OT_Operator)
    bpy.utils.unregister_class(OnloadOperator)
    del bpy.types.Scene.my_items
    del bpy.types.Scene.my_enum_prop
    del bpy.types.Scene.my_string_prop

if __name__ == "__main__":
    register()