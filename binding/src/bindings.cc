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
  *output_handle = reinterpret_cast<VWWorkspace*>(workspace.release());
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC void VWWorkspaceDelete(VWWorkspace* workspace_handle) noexcept {
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  delete workspace;
}

DLL_PUBLIC int VWWorkspaceLearn(VWWorkspace* workspace_handle, struct VWExample* example_handle, VWErrorMessage* error_message_handle) noexcept
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);

  workspace->learn(*ex);
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWWorkspaceGetPooledExample(struct VWWorkspace* workspace_handle, VWExample** output_handle, VWErrorMessage* error_message_handle) noexcept
{
  assert(workspace_handle != nullptr);
  assert(output_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace* >(workspace_handle);

  *output_handle = reinterpret_cast<VWExample*>(new_unused_example(*workspace));
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWWorkspaceReturnPooledExample(struct VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message_handle) noexcept
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace* >(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);
  empty_example(*workspace, *ex);
  workspace->example_parser->example_pool.return_object(ex);
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC struct VWExample* VWExampleCreate() noexcept
{
  return reinterpret_cast<VWExample*>(new VW::example);
}

DLL_PUBLIC void VWExampleDelete(struct VWExample* example_handle) noexcept
{
  auto* ex = reinterpret_cast<VW::example*>(example_handle);
  delete ex;
}
