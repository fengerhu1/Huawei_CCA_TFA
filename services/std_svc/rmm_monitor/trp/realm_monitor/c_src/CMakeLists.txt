cmake_minimum_required (VERSION 3.5)

# add_subdirectory("${REALM_C_SRC}/mm")

# add_library(${PROJECT_NAME}-c_src OBJECT
#     abs.c
#     $<TARGET_OBJECTS:${PROJECT_NAME}-mm>
# )

include_directories("${REALM_C_SRC}/include" "${REALM_C_SRC}/../../../../../../../mbedtls/include")

add_library(${PROJECT_NAME}-c_src OBJECT
    abs.c
    ns_access.S
    ContextSwitchAux.c
    mm/mm.c
    mm/mmu_helpers.S
)
