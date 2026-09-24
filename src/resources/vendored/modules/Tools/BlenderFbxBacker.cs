// -----------------------------------------------------
// Copyright (c) 2025 Erencan Pelin. All Rights Reserved.
// 
// Author: Erencan Pelin
// Date: 13/05/2025
// -----------------------------------------------------

using System;
using System.Diagnostics;
using System.IO;
using UnityEditor;
using UnityEngine;
using Debug = UnityEngine.Debug;

namespace TGP.Services.Editor
{
    public class BlenderFbxBaker : EditorWindow
    {
        private const string BlenderExePath = @"C:\Program Files\Blender Foundation\Blender 5.0/blender.exe";
        private const string ExportScriptPath = "blender_export_fbx.py";

        [MenuItem("Tools/Rebake All .blend to FBX")]
        private static void RebakeBlendFiles()
        {
            const string blenderExe = BlenderExePath;
            var exportScript = Path.Combine(Application.dataPath, "Scripts/Tools", ExportScriptPath);

            var blendSourceDir = Path.Combine(Application.dataPath, "Models", ".Blend");
            var fbxTargetDir = Path.Combine(Application.dataPath, "Models");

            var blendFiles = Directory.GetFiles(blendSourceDir, "*.blend", SearchOption.AllDirectories);

            foreach (var blendPath in blendFiles)
            {
                var blendName = Path.GetFileNameWithoutExtension(blendPath);
                var fbxOutput = Path.Combine(fbxTargetDir, blendName + ".fbx");

                if (File.Exists(fbxOutput) &&
                    File.GetLastWriteTimeUtc(blendPath) <= File.GetLastWriteTimeUtc(fbxOutput))
                {
                    Debug.Log($"[Skip] {blendName}.fbx is up to date.");
                    continue;
                }


                try
                {
                    var startInfo = new ProcessStartInfo
                    {
                        FileName = blenderExe,
                        Arguments = $"--background --python \"{exportScript}\" -- \"{blendPath}\" \"{fbxOutput}\"",
                        UseShellExecute = false,
                        RedirectStandardOutput = true,
                        RedirectStandardError = true,
                        CreateNoWindow = true
                    };

                    using var proc = new Process { StartInfo = startInfo };
                    proc.Start();

                    var stderr = proc.StandardError.ReadToEnd();

                    proc.WaitForExit();

                    if (!string.IsNullOrEmpty(stderr))
                        Debug.LogError($"[Blender Error] {blendName}:\n{stderr}");

                    if (File.Exists(fbxOutput))
                        Debug.Log($"[Rebake] ✅ {blendName}.fbx exported.");
                    else
                        Debug.LogError($"[Rebake] ❌ {blendName}.fbx not found after export.");
                }
                catch (Exception e)
                {
                    Debug.LogError($"[Rebake Error] {blendName}: {e.Message}");
                }
            }

            AssetDatabase.Refresh();
        }
    }
}