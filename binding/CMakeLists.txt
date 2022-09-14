cmake_minimum_required(VERSION 3.10)

set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)
set(CMAKE_VISIBILITY_INLINES_HIDDEN TRUE)
set(CMAKE_CXX_VISIBILITY_PRESET "hidden")

set(VCPKG_OVERLAY_PORTS "${CMAKE_CURRENT_LIST_DIR}/overlay-ports")

cmake_policy(SET CMP0091 NEW)

project(vowpalwabbit-rs-bindings LANGUAGES CXX)

set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

find_package(VowpalWabbit CONFIG REQUIRED)

add_library(vw_rs_bindings SHARED src/bindings.cc)
set_target_properties(vw_rs_bindings PROPERTIES PUBLIC_HEADER include/vw_rs_bindings/bindings.hpp)
target_compile_definitions(vw_rs_bindings PRIVATE VW_RS_BUILDING_DLL)
target_link_libraries(vw_rs_bindings PRIVATE VowpalWabbit::vw_core VowpalWabbit::vw_config VowpalWabbit::vw_common)
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
