#include "vw_rs_bindings/bindings.hpp"

#include <vw/core/action_score.h>
#include <vw/core/global_data.h>
#include <vw/core/prediction_type.h>

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

void* create_prediction(const polyprediction& prediction, VW::prediction_type_t pred_type)
{
  switch (pred_type)
  {
    case VW::prediction_type_t::scalar:
      THROW("VW::prediction_type_t::scalar is not supported")
      break;
    case VW::prediction_type_t::scalars:
      THROW("VW::prediction_type_t::scalars is not supported")
      break;
    case VW::prediction_type_t::action_scores:
    {
      auto* a_s = new ACTION_SCORE::action_scores;
      a_s->insert(a_s->begin(), prediction.a_s.begin(), prediction.a_s.end());
      return a_s;
    }
    break;
    case VW::prediction_type_t::pdf:
      THROW("VW::prediction_type_t::pdf is not supported")
      break;
    case VW::prediction_type_t::action_probs:
      THROW("VW::prediction_type_t::action_probs is not supported")
      break;
    case VW::prediction_type_t::multiclass:
      THROW("VW::prediction_type_t::multiclass is not supported")
      break;
    case VW::prediction_type_t::multilabels:
      THROW("VW::prediction_type_t::multilabels is not supported")
      break;
    case VW::prediction_type_t::prob:
      THROW("VW::prediction_type_t::prob is not supported")
      break;
    case VW::prediction_type_t::multiclassprobs:
      THROW("VW::prediction_type_t::multiclassprobs is not supported")
      break;
    case VW::prediction_type_t::decision_probs:
      THROW("VW::prediction_type_t::decision_probs is not supported")
      break;
    case VW::prediction_type_t::action_pdf_value:
      THROW("VW::prediction_type_t::action_pdf_value is not supported")
      break;
    case VW::prediction_type_t::active_multiclass:
      THROW("VW::prediction_type_t::active_multiclass is not supported")
      break;
    case VW::prediction_type_t::nopred:
      THROW("VW::prediction_type_t::nopred is not supported")
      break;
  }

  THROW("Unknown not supported")
}

VWErrorMessage* VWErrorMessageCreate() noexcept { return new VWErrorMessage; }
void VWErrorMessageDelete(VWErrorMessage* error_message) noexcept
{
  assert(error_message != nullptr);
  delete error_message;
}
const char* VWErrorMessageGetValue(const VWErrorMessage* error_message) noexcept
{
  assert(error_message != nullptr);
  return error_message->get();
}
void VWErrorMessageClearValue(VWErrorMessage* error_message) noexcept
{
  assert(error_message != nullptr);
  error_message->clear();
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
DLL_PUBLIC int VWWorkspaceSetupExample(
    const VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);

  auto* workspace = reinterpret_cast<const VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);

  // TODO: Ensure this is safe by disallowing write cache and kskipngram usage
  VW::setup_example(const_cast<VW::workspace&>(*workspace), ex);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspaceSetupMultiEx(
    const VWWorkspace* workspace_handle, VWMultiEx* example_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);

  auto* workspace = reinterpret_cast<const VW::workspace*>(workspace_handle);
  auto* multi_examples = reinterpret_cast<VW::multi_ex*>(example_handle);
  // TODO: Ensure this is safe by disallowing write cache and kskipngram usage
  VW::setup_examples(const_cast<VW::workspace&>(*workspace), *multi_examples);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspaceLearn(
    VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);

  workspace->learn(*ex);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspaceLearnMultiEx(
    VWWorkspace* workspace_handle, VWMultiEx* example_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  workspace->learn(*ex);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspacePredict(VWWorkspace* workspace_handle, VWExample* example_handle, void** prediction,
    uint32_t* prediction_type, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::example*>(example_handle);
  workspace->predict(*ex);
  *prediction_type = static_cast<uint32_t>(workspace->l->get_output_prediction_type());
  *prediction = create_prediction(ex->pred, workspace->l->get_output_prediction_type());
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspacePredictMultiEx(VWWorkspace* workspace_handle, VWMultiEx* example_handle, void** prediction,
    uint32_t* prediction_type, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(example_handle != nullptr);
  auto* workspace = reinterpret_cast<VW::workspace*>(workspace_handle);
  auto* ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  workspace->predict(*ex);
  *prediction_type = static_cast<uint32_t>(workspace->l->get_output_prediction_type());
  *prediction = create_prediction((*ex)[0]->pred, workspace->l->get_output_prediction_type());
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWWorkspaceParseDSJson(const VWWorkspace* workspace_handle, const char* json_string, size_t length,
    VWMultiEx* output_handle, VWErrorMessage* error_message) noexcept
try
{
  assert(workspace_handle != nullptr);
  assert(output_handle != nullptr);
  assert(json_string != nullptr);

  using example_factory_t = example& (*)(void*);

  example_factory_t factory = [](void* /* context */) -> VW::example& { return *new VW::example; };

  auto* workspace = const_cast<VW::workspace*>(reinterpret_cast<const VW::workspace*>(workspace_handle));
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

void VWMultiExDelete(VWMultiEx* multi_example_handle) noexcept
{
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(multi_example_handle);
  if (multi_example_handle == nullptr) { return; }

  for (auto* ex : *multi_ex) { delete ex; }
  delete multi_ex;
}

DLL_PUBLIC size_t VWMultiGetLength(const VWMultiEx* multi_example_handle) noexcept
{
  assert(multi_example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<const VW::multi_ex*>(multi_example_handle);
  return multi_ex->size();
}

DLL_PUBLIC int VWMultiGetExampleAt(
    VWMultiEx* multi_example_handle, VWExample** example, size_t index, VWErrorMessage* error_message) noexcept
{
  assert(multi_example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(multi_example_handle);
  if (multi_ex == nullptr)
  {
    error_message->set("nullptr");
    return VW_STATUS_FAIL;
  }
  if (index >= multi_ex->size())
  {
    error_message->set("out of bounds");
    return VW_STATUS_FAIL;
  }
  *example = reinterpret_cast<VWExample*>((*multi_ex)[index]);
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC int VWMultiReleaseExampleAt(
    VWMultiEx* multi_example_handle, VWExample** example, size_t index, VWErrorMessage* error_message) noexcept
try
{
  assert(multi_example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(multi_example_handle);
  if (multi_ex == nullptr)
  {
    error_message->set("nullptr");
    return VW_STATUS_FAIL;
  }
  if (index >= multi_ex->size())
  {
    error_message->set("out of bounds");
    return VW_STATUS_FAIL;
  }
  *example = reinterpret_cast<VWExample*>((*multi_ex)[index]);
  multi_ex->erase(multi_ex->begin() + index);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

// Deletes the example at that index
DLL_PUBLIC int VWMultiDeleteExampleAt(
    VWMultiEx* multi_example_handle, size_t index, VWErrorMessage* error_message) noexcept
try
{
  assert(multi_example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(multi_example_handle);
  if (multi_ex == nullptr)
  {
    error_message->set("nullptr");
    return VW_STATUS_FAIL;
  }
  if (index >= multi_ex->size())
  {
    error_message->set("out of bounds");
    return VW_STATUS_FAIL;
  }
  auto* example = (*multi_ex)[index];
  delete example;
  multi_ex->erase(multi_ex->begin() + index);
  return VW_STATUS_SUCCESS;
}
CATCH_RETURN_EXCEPTION

DLL_PUBLIC int VWMultiInsertExampleAt(
    VWMultiEx* example_handle, VWExample* example, size_t index, VWErrorMessage* error_message) noexcept
{
  assert(example_handle != nullptr);
  auto* multi_ex = reinterpret_cast<VW::multi_ex*>(example_handle);
  if (multi_ex == nullptr)
  {
    error_message->set("nullptr");
    return VW_STATUS_FAIL;
  }
  if (index > multi_ex->size())
  {
    error_message->set("out of bounds");
    return VW_STATUS_FAIL;
  }

  auto* ex = reinterpret_cast<VW::example*>(example);

  if (index == multi_ex->size()) { multi_ex->push_back(ex); }
  else { multi_ex->insert(multi_ex->begin() + index, ex); }
  return VW_STATUS_SUCCESS;
}

DLL_PUBLIC void VWActionScoresDelete(VWActionScores* action_scores_handle) noexcept
{
  assert(action_scores_handle != nullptr);
  auto* a_s = reinterpret_cast<ACTION_SCORE::action_scores*>(action_scores_handle);
  delete a_s;
}

DLL_PUBLIC void VWActionScoresGetLength(const VWActionScores* action_scores_handle, size_t* length) noexcept
{
  assert(action_scores_handle != nullptr);
  auto* a_s = reinterpret_cast<const ACTION_SCORE::action_scores*>(action_scores_handle);
  *length = a_s->size();
}

DLL_PUBLIC int VWActionScoresGetValue(const VWActionScores* action_scores_handle, uint32_t* action, float* value,
    size_t index, VWErrorMessage* error_message) noexcept
{
  assert(action_scores_handle != nullptr);
  auto& a_s = *reinterpret_cast<const ACTION_SCORE::action_scores*>(action_scores_handle);
  if (index >= a_s.size())
  {
    return VW_STATUS_FAIL;
    // TODO error message
  }

  *action = a_s[index].action;
  *value = a_s[index].score;
  return VW_STATUS_SUCCESS;
}