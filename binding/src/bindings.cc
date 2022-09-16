#include "vw_rs_bindings/bindings.hpp"

#include <iostream>
#include <string>
#include <vector>

#include "vw/common/string_view.h"
#include "vw/config/options.h"
#include "vw/config/options_cli.h"
#include "vw/core/example.h"
#include "vw/core/parse_example_json.h"
#include "vw/core/vw.h"
#include "vw/core/vw_fwd.h"

struct VWErrorMessage
{
  void set(VW::string_view message)
  {
    _is_set = true;
    _error_message = std::string{message};
  }

  // Acts like an optional. Either a string value or nullptr.
  const char* get() const
  {
    if (_is_set) { return _error_message.c_str(); }
    return nullptr;
  }

  void clear() { _is_set = false; }

private:
  bool _is_set = false;
  std::string _error_message;
};

#define CATCH_RETURN_EXCEPTION                                                 \
  catch (const std::exception& ex)                                             \
  {                                                                            \
    if (error_message != nullptr) { error_message->set(ex.what()); }           \
    return VW_STATUS_FAIL;                                                     \
  }                                                                            \
  catch (...)                                                                  \
  {                                                                            \
    if (error_message != nullptr) { error_message->set("Unknown exception"); } \
    return VW_STATUS_FAIL;                                                     \
  }

VWErrorMessage* VWErrorMessageCreate() noexcept { return new VWErrorMessage; }
void VWErrorMessageDelete(VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  delete error_message_handle;
}
const char* VWErrorMessageGetValue(const VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  return error_message_handle->get();
}
void VWErrorMessageClearValue(VWErrorMessage* error_message_handle) noexcept
{
  assert(error_message_handle != nullptr);
  error_message_handle->clear();
}

// VWWorkspace

DLL_PUBLIC int VWWorkspaceInitialize(
    const char* const* tokens, int count, VWWorkspace** output_handle, VWErrorMessage* error_message) noexcept
try
{
  std::vector<std::string> args(tokens, tokens + count);
  auto options = VW::make_unique<VW::config::options_cli>(args);
  auto workspace = VW::initialize_experimental(std::move(options));
  workspace->example_parser->strict_parse = true;
  *output_handle = reinterpret_cast<VWWorkspace*>(workspace.release());
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC void VWWorkspaceDelete(VWWorkspace* workspace_handle) noexcept
{
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  delete workspace;
}

DLL_PUBLIC int VWWorkspaceLearn(
    VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message_handle) noexcept
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);

  workspace->learn(*ex);
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWWorkspaceParseDSJson(VWWorkspace* workspace_handle, const char* json_string, size_t length,
    VWMultiEx* output_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(output_handle != nullptr);
  assert(json_string != nullptr);

  using example_factory_t = example& (*)(void*);

  example_factory_t factory = [](void* /* context */) -> VW::example& { return *new VW::example; };

  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(output_handle);
  assert(multi_ex->empty());
  multi_ex->push_back(new VW::example);
  DecisionServiceInteraction info;
  VW::read_line_decision_service_json<false>(
      *workspace, *multi_ex, const_cast<char*>(json_string), length, true, factory, nullptr, &info);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC VWExample* VWExampleCreate() noexcept { return reinterpret_cast<VWExample*>(new VW::example); }

DLL_PUBLIC void VWExampleDelete(VWExample* example_handle) noexcept
{
  auto* ex = reinterpret_cast<VW::example*>(example_handle);
  delete ex;
}

VWMultiEx* VWMultiExCreate() noexcept { return reinterpret_cast<VWMultiEx*>(new VW::multi_ex); }

void VWMultiExDelete(VWMultiEx* example_handle) noexcept
{
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  if (example_handle == nullptr) { return; }

  for (auto* ex : *multi_ex) { delete ex; }
  delete multi_ex;
}

// Just returns a view into the list
void VWMultiGetExample(VWMultiEx* example_handle, void** examples, size_t* length) noexcept
{
  assert(example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  *examples = reinterpret_cast<void*>(multi_ex->data());
  *length = multi_ex->size();
}

DLL_PUBLIC size_t VWMultiGetLength(VWMultiEx* example_handle) noexcept
{
  assert(example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  return multi_ex->size();
}

DLL_PUBLIC int VWMultiGetExample(
    VWMultiEx* example_handle, VWExample** example, size_t index, VWErrorMessage* error_message_handle) noexcept
{
  assert(example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  if (multi_ex == nullptr)
  {
    error_message_handle->set("nullptr");
    return VW_STATUS_FAIL;
  }
  if (index >= multi_ex->size())
  {
    error_message_handle->set("out of bounds");
    return VW_STATUS_FAIL;
  }
  *example = reinterpret_cast<VWExample*>((*multi_ex)[index]);
  return VW_STATUS_SUCCESS;
}

// Does not delete the contained examples.
void VWMultiClear(VWMultiEx* example_handle) noexcept
{
  assert(example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  multi_ex->clear();
}
