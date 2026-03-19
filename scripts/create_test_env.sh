mkdir TestProject
mkdir TestProject/Assets
mkdir TestProject/ProjectSettings
mkdir TestProject/Packages

# populate project settings
echo "PlayerSettings:
  companyName: ErencanPelin
  productName: TestProject" > TestProject/ProjectSettings/ProjectSettings.asset

cat scripts/test_manifest.json > TestProject/Packages/manifest.json