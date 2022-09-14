#include "vw_rs_bindings/bindings.hpp"

#include <iostream>
#include <string>
#include <vector>

#include "vw/common/string_view.h"
#include "vw/config/options.h"
#include "vw/config/options_cli.h"
#include "vw/core/vw.h"

struct VWErrorMessage {
  void set(VW::string_view message) {
    _is_set = true;
    _error_message = std::string{message}; }

  // Acts like an optional. Either a string value or nullptr.
  const char* get() const {
    if (_is_set){
    return _error_message.c_str();
    }
    return nullptr;
    }

  void clear()
  {
    _is_set = false;
  }

private:
  bool _is_set = false;
  std::string _error_message;
};

#define CATCH_RETURN_EXCEPTION                                                 \
  catch (const std::exception &ex) {                                           \
    if (error_message != nullptr) {                                            \
      error_message->set(ex.what());                                            \
    }                                                                          \
    return VW_STATUS_FAIL;                                                     \
  }                                                                            \
  catch (...) {                                                                \
    if (error_message != nullptr) {                                            \
      error_message->set("Unknown exception");                                  \
    }                                                                          \
    return VW_STATUS_FAIL;                                                     \
  }

DLL_PUBLIC struct VWErrorMessage* VWErrorMessageCreate() noexcept
{
  return new VWErrorMessage;
}
DLL_PUBLIC void VWErrorMessageDelete(struct VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  delete error_message_handle;
}
DLL_PUBLIC const char* VWErrorMessageGetValue(const struct VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  return error_message_handle->get();
}
DLL_PUBLIC void VWErrorMessageClearValue(struct VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  error_message_handle->clear();
}

// VWWorkspace

DLL_PUBLIC int VWWorkspaceInitialize(const char* const* tokens, int count,
                                     VWWorkspace** output_handle,
                                     VWErrorMessage* error_message) noexcept
    try {
  std::vector<std::string> args(tokens, tokens + count);
  auto options = VW::make_unique<VW::config::options_cli>(args);
  auto workspace = VW::initialize_experimental(std::move(options));
  *output_handle = reinterpret_cast<VWWorkspace* >(workspace.release());
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC void VWWorkspaceDelete(VWWorkspace* workspace_handle) noexcept {
  auto* workspace = reinterpret_cast<VW::workspace* >(workspace_handle);
  delete workspace;
}
