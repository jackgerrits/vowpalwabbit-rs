#pragma once

#include <cstddef>
#include <cstdint>

#if defined _WIN32 || defined __CYGWIN__
#  ifdef VW_RS_BUILDING_DLL
#    ifdef __GNUC__
#      define DLL_PUBLIC __attribute__((dllexport))
#    else
#      define DLL_PUBLIC __declspec(dllexport)  // Note: actually gcc seems to also supports this syntax.
#    endif
#  else
#    ifdef __GNUC__
#      define DLL_PUBLIC __attribute__((dllimport))
#    else
#      define DLL_PUBLIC __declspec(dllimport)  // Note: actually gcc seems to also supports this syntax.
#    endif
#  endif
#  define DLL_LOCAL
#else
#  if __GNUC__ >= 4
#    define DLL_PUBLIC __attribute__((visibility("default")))
#    define DLL_LOCAL __attribute__((visibility("hidden")))
#  else
#    define DLL_PUBLIC
#    define DLL_LOCAL
#  endif
#endif

// For operations which cannot fail under any circumstance (except out of memory) it is acceptable to omit the return
// code, and error holder. If it is an operation which can fail, it must return an error code and accept the error
// message parameter for filling with failure info.

extern "C"
{
  static const int VW_STATUS_SUCCESS = 0;
  static const int VW_STATUS_FAIL = 1;

  // Unfortunately a copy paste of the enum since bringing in the header is not
  // feasible and using externs would mean these are no longer constants
  enum class override_prediction_type_t : uint32_t
  {
    scalar,
    scalars,
    action_scores,
    pdf,
    action_probs,
    multiclass,
    multilabels,
    prob,
    multiclassprobs,  // not in use (technically oaa.cc)
    decision_probs,
    action_pdf_value,
    active_multiclass,
    nopred
  };

  struct VWWorkspace;
  struct VWExample;
  struct VWErrorMessage;
  struct VWMultiEx;

  struct VWActionScores;

  DLL_PUBLIC VWErrorMessage* VWErrorMessageCreate() noexcept;
  DLL_PUBLIC void VWErrorMessageDelete(VWErrorMessage* error_message) noexcept;
  // If there was no error message set, a nullptr is returned.
  DLL_PUBLIC const char* VWErrorMessageGetValue(const VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC void VWErrorMessageClearValue(VWErrorMessage* error_message) noexcept;

  DLL_PUBLIC void VWWorkspaceDeleteBuffer(const unsigned char* buffer) noexcept;

  DLL_PUBLIC int VWWorkspaceInitialize(
      const char* const* tokens, size_t count, VWWorkspace** output_handle, VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC int VWWorkspaceInitializeFromModel(
      const char* const* extra_tokens, size_t count, const unsigned char* bytes, size_t num_bytes, VWWorkspace** output_handle, VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC void VWWorkspaceDelete(VWWorkspace* workspace_handle) noexcept;

  // bytes must be deleted using delete buffer
  DLL_PUBLIC int VWWorkspaceSerializeModel(const VWWorkspace* workspace_handle, const unsigned char** bytes, size_t* num_bytes, VWErrorMessage* error_message) noexcept;
  // bytes is a c string and must be deleted using delete buffer
  DLL_PUBLIC int VWWorkspaceSerializeReadableModel(const VWWorkspace* workspace_handle, const unsigned char** bytes, size_t* num_bytes, VWErrorMessage* error_message) noexcept;

  DLL_PUBLIC int VWWorkspaceSetupExample(
      const VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC int VWWorkspaceSetupMultiEx(
      const VWWorkspace* workspace_handle, VWMultiEx* example_handle, VWErrorMessage* error_message) noexcept;

  DLL_PUBLIC int VWWorkspaceLearn(
      VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC int VWWorkspaceLearnMultiEx(
      VWWorkspace* workspace_handle, VWMultiEx* example_handle, VWErrorMessage* error_message) noexcept;
  // Will allocate a prediction based on the returned prediction_type. It must be deleted with the corresponding type
  // deleter.
  // TODO: tackle fact that predict sets test_only meaning that it is no longer able to be used in learn
  DLL_PUBLIC int VWWorkspacePredict(VWWorkspace* workspace_handle, VWExample* example_handle, void** prediction,
      uint32_t* prediction_type, VWErrorMessage* error_message) noexcept;
  // Will allocate a prediction based on the returned prediction_type. It must be deleted with the corresponding type
  // deleter.
  DLL_PUBLIC int VWWorkspacePredictMultiEx(VWWorkspace* workspace_handle, VWMultiEx* example_handle, void** prediction,
      uint32_t* prediction_type, VWErrorMessage* error_message) noexcept;

  typedef VWExample* VWExampleFactoryFunc(void*);
  DLL_PUBLIC int VWWorkspaceParseDSJson(const VWWorkspace* workspace_handle, const char* json_string, size_t length, VWExampleFactoryFunc example_factory, void* example_factory_context,
      VWMultiEx* output_handle,  VWErrorMessage* error_message) noexcept;

  DLL_PUBLIC VWExample* VWExampleCreate() noexcept;
  DLL_PUBLIC void VWExampleDelete(VWExample* example_handle) noexcept;
  DLL_PUBLIC void VWExampleClear(VWExample* example_handle) noexcept;

  DLL_PUBLIC VWMultiEx* VWMultiExCreate() noexcept;
  // If any examples are held in the container they will be deleted too.
  DLL_PUBLIC void VWMultiExDelete(VWMultiEx* example_handle) noexcept;
  DLL_PUBLIC size_t VWMultiGetLength(const VWMultiEx* example_handle) noexcept;
  // Returns a pointer to that example.
  DLL_PUBLIC int VWMultiGetExampleAt(
      VWMultiEx* example_handle, VWExample** example, size_t index, VWErrorMessage* error_message) noexcept;
  // Releases the example at the index. Removes it from the collection and its lifetime must be managed by the caller.
  DLL_PUBLIC int VWMultiReleaseExampleAt(
      VWMultiEx* example_handle, VWExample** example, size_t index, VWErrorMessage* error_message) noexcept;
  // Deletes the example at that index
  DLL_PUBLIC int VWMultiDeleteExampleAt(
      VWMultiEx* example_handle, size_t index, VWErrorMessage* error_message) noexcept;
  // Lifetime transfers to the multiex. Use index == size to push at end.
  DLL_PUBLIC int VWMultiInsertExampleAt(
      VWMultiEx* example_handle, VWExample* example, size_t index, VWErrorMessage* error_message) noexcept;

  DLL_PUBLIC void VWActionScoresDelete(VWActionScores* action_scores_handle) noexcept;
  DLL_PUBLIC void VWActionScoresGetLength(const VWActionScores* action_scores_handle, size_t* length) noexcept;
  DLL_PUBLIC int VWActionScoresGetValue(const VWActionScores* action_scores_handle, uint32_t* action, float* value,
      size_t index, VWErrorMessage* error_message) noexcept;
}
