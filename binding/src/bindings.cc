#include "vw_rs_bindings/bindings.hpp"

#include <iostream>
#include <string>
#include <vector>

#include "vw/config/options.h"
#include "vw/config/options_cli.h"
#include "vw/core/vw.h"

const char* copy_cstr(const char* str)
{
  if (str == nullptr)
  {
    return nullptr;
  }
  const auto len = std::strlen(str);
  char* str_copy = new char[len + 1];
  std::strncpy(str_copy, str, len);
  str_copy[len] = '\0';
  return str_copy;
}

#define CATCH_RETURN_EXCEPTION                       \
  catch (const std::exception& ex)                   \
  {                                                  \
    *error_message = copy_cstr(ex.what());           \
    return VW_STATUS_FAIL;                           \
  }                                                  \
  catch (...)                                        \
  {                                                  \
    *error_message = copy_cstr("Unknown exception"); \
    return VW_STATUS_FAIL;                           \
  }

DLL_PUBLIC int VWInitializeWorkspace(
    const char* const* symbols, int count, VWWorkspace** output_handle, const char** error_message) noexcept
{
  try
  {
    std::vector<std::string> args(symbols, symbols + count);
    auto options = VW::make_unique<VW::config::options_cli>(args);
    auto workspace = VW::initialize_experimental(std::move(options));
    *output_handle = reinterpret_cast<VWWorkspace*>(workspace.release());
  }
  CATCH_RETURN_EXCEPTION

  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWFreeWorkspace(VWWorkspace* workspace_handle, const char** error_message) noexcept
{
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  delete workspace;
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWFreeErrorMessage(const char* error_message) noexcept
{
  delete[] error_message;
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWRunDriver(VWWorkspace* workspace_handle, const char** error_message) noexcept
{
  auto& workspace = *reinterpret_cast<VW::workspace*>(workspace_handle);

  try
  {
    VW::start_parser(workspace);
    VW::LEARNER::generic_driver(workspace);
    VW::end_parser(workspace);
    VW::sync_stats(workspace);
    VW::finish(workspace, false);
  }
  CATCH_RETURN_EXCEPTION

  return VW_STATUS_SUCCESS;
}
