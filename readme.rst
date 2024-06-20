Trusted Firmware-A
==================

Trusted Firmware-A (TF-A) is a reference implementation of secure world software
for `Arm A-Profile architectures`_ (Armv8-A and Armv7-A), including an Exception
Level 3 (EL3) `Secure Monitor`_. It provides a suitable starting point for
productization of secure world boot and runtime firmware, in either the AArch32
or AArch64 execution states.

TF-A implements Arm interface standards, including:

-  `Power State Coordination Interface (PSCI)`_
-  `Trusted Board Boot Requirements CLIENT (TBBR-CLIENT)`_
-  `SMC Calling Convention`_
-  `System Control and Management Interface (SCMI)`_
-  `Software Delegated Exception Interface (SDEI)`_

The code is designed to be portable and reusable across hardware platforms and
software models that are based on the Armv8-A and Armv7-A architectures.

In collaboration with interested parties, we will continue to enhance TF-A
with reference implementations of Arm standards to benefit developers working
with Armv7-A and Armv8-A TrustZone technology.

Users are encouraged to do their own security validation, including penetration
testing, on any secure world code derived from TF-A.

Compilation Guide
-----------------

Common Prerequisites
^^^^^^^^^^^^^^^^^^^^

- `cca_build_v7` docker image

Now the repo is available for `qemu` and `fvp` platforms.

QEMU
^^^^

Prerequisites
~~~~~~~~~~~~~

docker_cmd.sh
^^^^^^^^^^^^^

.. code-block:: bash

    #!/bin/bash
    # run docker 
    if [[ $1 == *"docker"* ]]; then
        echo "Run: run docker"
        docker run -p 54340:54340 -v $(pwd):/home/mount_dir -w /home/mount_dir --rm -it cca_build_v7:latest bash
        exit 0
   


source.sh
^^^^^^^^^

.. code-block:: bash

    export CROSS_COMPILE=aarch64-none-elf-
    export PATH=/home/mount_dir/toolchains/gcc-arm-10.3-2021.07-x86_64-aarch64-none-elf/bin:$PATH

tfa.sh
^^^^^^

.. code-block:: bash

    cd trusted-firmware-a/services/std_svc/rmm_monitor/trp/realm_monitor
    cmake -DPLAT=qemu
    make -C .
    cd ../../../../../..
    source source.sh
    cd trusted-firmware-a
    rm -r build/qemu
    make CROSS_COMPILE=aarch64-none-elf- ARCH=aarch64 PLAT=qemu ARM_DISABLE_TRUSTED_WDOG=1 ENABLE_RME=1 DEBUG=1 BL33=../edk2/Build/ArmVirtQemuKernel-AARCH64/DEBUG_GCC5/FV/QEMU_EFI.fd all fip


Build edk2
~~~~~~~~~~

You need to build edk2 at the parent directory of TF-A repo.

.. code-block:: bash

    git clone https://github.com/tianocore/edk2.git
    cd edk2
    git submodule update --init
    sudo ./docker_cmd.sh docker
        update-alternatives --install /usr/bin/python python /usr/bin/python3 1
        make -C BaseTools
        source edksetup.sh
        export GCC5_AARCH64_PREFIX=/home/mount_dir/toolchains/aarch64/bin/aarch64-linux-gnu-
        build -a AARCH64 -t GCC5 -p ArmVirtPkg/ArmVirtQemuKernel.dsc
        exit

Build TF-A
~~~~~~~~~~

Make sure you are in the docker container.

.. code-block:: bash

    ./docker_cmd docker

Then you can build TF-A with the following command.

.. code-block:: bash

    ./tfa.sh

FVP
^^^

You can just build TF-A with the `fvp.mk` in the `build` directory of rme repo.

First you need to enter docker container.
Run the following command in the root of rme repo to enter docker container.

.. code-block:: bash

    ./docker_cmd docker

Then you can build TF-A with the following command.

.. code-block:: bash

    cd build
    make

More Info and Documentation
---------------------------

To find out more about Trusted Firmware-A, please `view the full documentation`_
that is available through `trustedfirmware.org`_.

--------------

*Copyright (c) 2013-2019, Arm Limited and Contributors. All rights reserved.*

.. _Armv7-A and Armv8-A: https://developer.arm.com/products/architecture/a-profile
.. _Secure Monitor: http://www.arm.com/products/processors/technologies/trustzone/tee-smc.php
.. _Power State Coordination Interface (PSCI): PSCI_
.. _PSCI: http://infocenter.arm.com/help/topic/com.arm.doc.den0022d/Power_State_Coordination_Interface_PDD_v1_1_DEN0022D.pdf
.. _Trusted Board Boot Requirements CLIENT (TBBR-CLIENT): https://developer.arm.com/docs/den0006/latest/trusted-board-boot-requirements-client-tbbr-client-armv8-a
.. _SMC Calling Convention: http://infocenter.arm.com/help/topic/com.arm.doc.den0028b/ARM_DEN0028B_SMC_Calling_Convention.pdf
.. _System Control and Management Interface (SCMI): SCMI_
.. _SCMI: http://infocenter.arm.com/help/topic/com.arm.doc.den0056a/DEN0056A_System_Control_and_Management_Interface.pdf
.. _Software Delegated Exception Interface (SDEI): SDEI_
.. _SDEI: http://infocenter.arm.com/help/topic/com.arm.doc.den0054a/ARM_DEN0054A_Software_Delegated_Exception_Interface.pdf
.. _Arm A-Profile architectures: https://developer.arm.com/architectures/cpu-architecture/a-profile
.. _view the full documentation: https://www.trustedfirmware.org/docs/tf-a
.. _trustedfirmware.org: http://www.trustedfirmware.org

