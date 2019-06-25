"./flatc.exe" --cpp --rust --force-empty svc_mesh.fbs
cp ./svc_mesh_generated.rs ../src/generated.rs