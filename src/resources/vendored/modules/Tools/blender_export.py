import os
import sys

import bpy

"""
Allows exporting a .blend file to .fbx format using Blender's Python API. 
The script takes two command-line arguments: the path to the input .blend file and the path to the output .fbx file. 
It opens the specified .blend file, selects all objects, and exports them to the specified .fbx file with appropriate settings for scale and axis orientation.

This is especially useful because if you're running a Unity build from a pipeline, it will fail to export .blend files during the build process
unless you install Blender on the build machine.

This script can be used in a build pipeline to automate the conversion of .blend files to .fbx format, ensuring that the assets are correctly exported and can be used in Unity without manual intervention.
"""

blend_file = sys.argv[-2]
fbx_output = sys.argv[-1]

bpy.ops.wm.open_mainfile(filepath=blend_file)
if bpy.context.object and bpy.context.object.mode != 'OBJECT':
    bpy.ops.object.mode_set(mode='OBJECT')
bpy.ops.object.select_all(action='SELECT')
bpy.ops.export_scene.fbx(
    filepath=fbx_output,
    use_selection=True,
    apply_unit_scale=True,
    apply_scale_options='FBX_SCALE_ALL',
    global_scale=1.0,
    axis_forward='-Z',
    axis_up='Y',
    use_mesh_modifiers=True,
    bake_space_transform=True
)