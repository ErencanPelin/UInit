rm -rf TestProject/Assets
rm -rf TestProject/uinitool.toml

# reset unity project files
echo "PlayerSettings:
  companyName: ErencanPelin
  productName: TestProject" > TestProject/ProjectSettings/ProjectSettings.asset

cat scripts/test_manifest.json > TestProject/Packages/manifest.json