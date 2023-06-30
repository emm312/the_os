#!/usr/bin/env python3
import os

def log_info(msg):
    """
    Logs a message with info log level.
    """
    print(f"\033[1m\033[92minfo\033[0m: {msg}")


def log_error(msg):
    """
    Logs a message with error log level.
    """
    print(f"\033[1m\033[91merror\033[0m: {msg}")


OVMF_URL = 'https://github.com/aero-os/ovmf-prebuilt'
LIMINE_URL = 'https://github.com/limine-bootloader/limine'

LIMINE_TEMPLATE = """
TIMEOUT=0
VERBOSE=yes

:theos
PROTOCOL=limine
KERNEL_PATH=boot:///the_os.elf
RESOLUTION=1000x1000
"""

os.system("mkdir build")

if not os.path.exists("build/ovmf-prebuilt"):
    os.system(f"git clone {OVMF_URL} --depth 1 build/ovmf-prebuilt")
if not os.path.exists("build/limine"):
    os.system(f"git clone {LIMINE_URL} --branch v5.x-branch-binary --depth 1 build/limine")
        
os.system("gmake -C build/limine")

os.system("cargo b")
os.system("mv ./target/x86_64-unknown-none/debug/the_os ./build/the_os.elf")

log_info("preparing ISO")

os.system("mkdir build/iso_root")
os.system("cp ./build/the_os.elf ./build/iso_root")
os.system("cp build/limine/limine-bios.sys build/limine/limine-bios-cd.bin build/limine/limine-uefi-cd.bin build/iso_root/")
os.system("touch build/iso_root/limine.cfg")
with open("build/iso_root/limine.cfg", "w") as file:
    file.write(LIMINE_TEMPLATE)
os.system("mkdir -p build/iso_root/EFI/BOOT")
os.system("cp build/limine/BOOTX64.efi ./build/iso_root/EFI/BOOT")
os.system("xorriso -as mkisofs -b limine-bios-cd.bin \
		-no-emul-boot -boot-load-size 4 -boot-info-table \
		--efi-boot limine-uefi-cd.bin \
		-efi-boot-part --efi-boot-image --protective-msdos-label \
		build/iso_root -o the_os.iso")
os.system("./build/limine/limine bios-install the_os.iso")

os.system("rm -rf build/iso_root")
log_info("Running the OS")
os.system("qemu-system-x86_64 -M q35 -m 2G -bios build/ovmf-prebuilt/ovmf-x86_64/OVMF.fd -cdrom the_os.iso -boot d -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio")
