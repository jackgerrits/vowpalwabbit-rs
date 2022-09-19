vcpkg_check_linkage(ONLY_STATIC_LIBRARY)

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO VowpalWabbit/vowpal_wabbit
    REF ebf709fa2a4f9d91b51bce1c49cc684d549ac2ff
    SHA512 777c9a95458e18404d77fd1a1dd478bbcd9ee5dac88fc3f1de15eedcf218895dd2ed2269024a4b3acf4336b8a16aa6e4b18680cbeadb7852a2706d03c364bbbb
    HEAD_REF master
)

vcpkg_cmake_configure(
    SOURCE_PATH ${SOURCE_PATH}
    OPTIONS
        -DSTATIC_LINK_VW_JAVA=ON
        -DVW_INSTALL=ON
        -DRAPIDJSON_SYS_DEP=ON
        -DFMT_SYS_DEP=ON
        -DSPDLOG_SYS_DEP=ON
        -DVW_BOOST_MATH_SYS_DEP=ON
        -DVW_ZLIB_SYS_DEP=ON
        -DVW_BUILD_VW_C_WRAPPER=OFF
        -DBUILD_TESTING=OFF
)
vcpkg_cmake_install()

file(REMOVE_RECURSE "${CURRENT_PACKAGES_DIR}/debug/include")

file(INSTALL "${SOURCE_PATH}/LICENSE" DESTINATION "${CURRENT_PACKAGES_DIR}/share/${PORT}" RENAME copyright)

vcpkg_cmake_config_fixup(CONFIG_PATH lib/cmake/VowpalWabbit/)

# hack: This is only build on non-windows. For now just add this.
set(active_interactor_bin "")
if(NOT WIN32)
    set(active_interactor_bin "active_interactor")
endif()

vcpkg_copy_tools(TOOL_NAMES vw spanning_tree ${active_interactor_bin} AUTO_CLEAN)
