
cmake_minimum_required (VERSION 3.5)

set(CMAKE_VERBOSE_MAKEFILE on)

set(ROOTDIR "${CMAKE_SOURCE_DIR}")

set(CROSS_COMPILE "/home/tiger/workspace/arm_enclave/twinvisor-prototype/toolchains/aarch64/bin/aarch64-linux-gnu-")

set(CMAKE_C_COMPILER "${CROSS_COMPILE}gcc")
set(CMAKE_CPP_COMPILER "${CROSS_COMPILE}cpp")
set(CMAKE_ASM_COMPILER "${CROSS_COMPILE}gcc")
set(CMAKE_OBJ_COPY "${CROSS_COMPILE}objcopy")

# set(CMAKE_C_FLAGS "-std=gnu99 -fdiagnostics-show-option -Wall -Werror-implicit-function-declaration -nostdinc -nostdlib -march=armv8.4-a -nostartfiles -g" CACHE STRING "" FORCE)

# SET(CMAKE_ASM_FLAGS "${CMAKE_C_FLAGS} -D__ASSEMBLY__" CACHE STRING "" FORCE)

# set(CMAKE_GEN_ASM_FLAGS "-std=gnu99 -fdiagnostics-show-option -Wall -Werror-implicit-function-declaration -nostdinc -nostdlib -march=armv8.4-a -nostartfiles -S" CACHE STRING "" FORCE)


set(CMAKE_C_FLAGS "-std=gnu99 -fdiagnostics-show-option -Wall -Werror-implicit-function-declaration -nostdlib -march=armv8.4-a -nostartfiles -g" CACHE STRING "" FORCE)

if(PLAT STREQUAL "qemu")
    add_definitions(-DPLAT_QEMU)
elseif(PLAT STREQUAL "fvp")
    add_definitions(-DPLAT_FVP)
else()
    message(FATAL_ERROR "Unsupported platform: ${PLAT}")
endif()

SET(CMAKE_ASM_FLAGS "${CMAKE_C_FLAGS} -D__ASSEMBLY__" CACHE STRING "" FORCE)

set(CMAKE_GEN_ASM_FLAGS "-std=gnu99 -fdiagnostics-show-option -Wall -Werror-implicit-function-declaration -nostdlib -march=armv8.4-a -nostartfiles -S" CACHE STRING "" FORCE)

project (RMM C ASM)

set(REALM_C_SRC          "${CMAKE_SOURCE_DIR}/c_src")

include_directories("${CMAKE_SOURCE_DIR}/../../../../include")
include_directories("${CMAKE_SOURCE_DIR}/../../../../include/arch/aarch64")
include_directories("${CMAKE_SOURCE_DIR}/../../../../plat/arm/board/fvp/include/")

# add_subdirectory("${REALM_C_SRC}")

# set(LINKER_SCRIPT "${CMAKE_SOURCE_DIR}/c_src/linker.lds")

set(LINK_SCRIPT "linker.lds")


# set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -T ${LINKER_SCRIPT}")


add_executable(rmm
    trp_entry.S
    trp_main.c
    trp_helpers.c
)

set_property(
    TARGET rmm
    APPEND_STRING
    PROPERTY
        LINK_FLAGS
        "-T ${CMAKE_CURRENT_BINARY_DIR}/${LINK_SCRIPT}"
)

set_property(
    TARGET rmm
    APPEND_STRING
    PROPERTY
        DEPENDS
        "${LINK_SCRIPT}"
)

# set_target_properties(realm PROPERTIES LINK_DEPENDS ${LINKER_SCRIPT})


# add_custom_target(
#     librust_rmm
#     BYPRODUCTS "${CMAKE_SOURCE_DIR}/target/aarch64-unknown-none-softfloat/release/librealm_monitor.a"
#     COMMAND sh ${CMAKE_SOURCE_DIR}/build.sh
#     WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
# )

# add_dependencies(realm librust_rmm)


# add_custom_command(
#     TARGET rmm
#     PRE_LINK
#     COMMAND ${CMAKE_CPP_COMPILER} -D__ASSEMBLY__ -P -C 
#     -I${CMAKE_SOURCE_DIR}/../../../../include
#     ${CMAKE_SOURCE_DIR}/c_src/linker.ld.S -o ${LINK_SCRIPT}
# )

add_custom_command(
    TARGET rmm
    POST_BUILD
    COMMAND ${CMAKE_OBJ_COPY} -O binary -S rmm rmm.bin
)

# target_link_libraries(realm "${CMAKE_SOURCE_DIR}/target/aarch64-unknown-none-softfloat/release/librealm_monitor.a")

