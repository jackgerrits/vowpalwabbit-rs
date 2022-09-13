cmake_minimum_required(VERSION 3.7)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CXX_STANDARD 11)
SET(VW_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_VISIBILITY_INLINES_HIDDEN TRUE)
set(CMAKE_CXX_VISIBILITY_PRESET "hidden")

cmake_policy(SET CMP0091 NEW)

project(vowpalwabbit-rs-bindings LANGUAGES CXX)

set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

set(STATIC_LINK_VW_JAVA ON CACHE BOOL "" FORCE)
set(VW_INSTALL OFF CACHE BOOL "" FORCE)
set(RAPIDJSON_SYS_DEP ON CACHE BOOL "" FORCE)
set(FMT_SYS_DEP ON CACHE BOOL "" FORCE)
set(SPDLOG_SYS_DEP ON CACHE BOOL "" FORCE)
set(VW_BOOST_MATH_SYS_DEP ON CACHE BOOL "" FORCE)
set(VW_ZLIB_SYS_DEP ON CACHE BOOL "" FORCE)
set(BUILD_TESTING OFF CACHE BOOL "" FORCE)

add_subdirectory(external/vowpal_wabbit EXCLUDE_FROM_ALL)

add_library(vw_rs_bindings SHARED src/bindings.cc)
set_target_properties(vw_rs_bindings PROPERTIES PUBLIC_HEADER include/vw_rs_bindings/bindings.hpp)
target_compile_definitions(vw_rs_bindings PRIVATE VW_RS_BUILDING_DLL)
target_link_libraries(vw_rs_bindings PRIVATE vw_core vw_config)
target_include_directories(vw_rs_bindings PUBLIC include)

include(GNUInstallDirs)
install(
  TARGETS vw_rs_bindings
  LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
  ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
  RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
  PUBLIC_HEADER DESTINATION ${CMAKE_INSTALL_INCLUDEDIR}/vw_rs_bindings
)

set(VW_VERSION "${VW_VERSION_MAJOR}.${VW_VERSION_MINOR}.${VW_VERSION_PATCH}")
configure_file(vw_rs_bindings.pc.in vw_rs_bindings.pc @ONLY)
install(
  FILES ${CMAKE_CURRENT_BINARY_DIR}/vw_rs_bindings.pc
  DESTINATION ${CMAKE_INSTALL_LIBDIR}/pkgconfig
)
