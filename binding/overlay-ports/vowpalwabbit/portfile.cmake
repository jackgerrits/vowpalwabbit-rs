vcpkg_check_linkage(ONLY_STATIC_LIBRARY)

vcpkg_from_github(
    OUT_SOURCE_PATH SOURCE_PATH
    REPO jackgerrits/vowpal_wabbit
    REF e5f8d2804854f9ba51e7c3edbc562bce711f5074
    SHA512 7948995578480c5923e9eafe5f135958010d8c740595bdd8f25000020ac522d3cd002a52c0fc7e3de2b78000c4c728b62c1b2399f647f9bfbea13f2ffd538cf5
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
