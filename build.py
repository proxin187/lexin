import os

print("[ Build ] cargo build --release")
os.system("cargo build --release")
print("[ Local Install ] mv target/debug/lexin lexin")
os.system("mv target/release/lexin lexin")


