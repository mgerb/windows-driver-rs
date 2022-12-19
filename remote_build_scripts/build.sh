# code is compiled on this machine
build_ip=192.168.122.66;
build_user="mg";
# executable is copied to this machine
target_ip=192.168.122.195;
target_user="mg";

# output file name
driver_name="driver.sys"

# clean build files from build machine
ssh "$build_user@$build_ip" "rmdir /s /Q C:\Users\\$build_user\Desktop\windows-driver-rs";

# copy source code to build machine
scp -r ../windows-driver-rs "$build_user@$build_ip:/C:/Users/$build_user/Desktop/windows-driver-rs";

# build driver
ssh "$build_user@$build_ip" "cd C:\Users\\$build_user\Desktop\windows-driver-rs\driver && cargo build --features winreg";

# copy from build machine to test machine
scp "$build_user@$build_ip:/C:/Users/$build_user/Desktop/windows-driver-rs/target/x86_64-pc-windows-msvc/debug/driver.dll" "$target_user@$target_ip:/C:/Users/$target_user/Desktop/$driver_name";

