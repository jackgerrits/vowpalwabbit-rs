#pragma once

#include <cstddef>

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

  struct VWWorkspace;
  struct VWExample;
  struct VWErrorMessage;
  struct VWMultiEx;

  DLL_PUBLIC VWErrorMessage* VWErrorMessageCreate() noexcept;
  DLL_PUBLIC void VWErrorMessageDelete(VWErrorMessage* error_message_handle) noexcept;
  // If there was no error message set, a nullptr is returned.
  DLL_PUBLIC const char* VWErrorMessageGetValue(const VWErrorMessage* error_message_handle) noexcept;
  DLL_PUBLIC void VWErrorMessageClearValue(VWErrorMessage* error_message_handle) noexcept;

  DLL_PUBLIC int VWWorkspaceInitialize(
      const char* const* tokens, int count, VWWorkspace** output_handle, VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC void VWWorkspaceDelete(VWWorkspace* workspace_handle) noexcept;

  DLL_PUBLIC int VWWorkspaceLearn(
      VWWorkspace* workspace_handle, VWExample* example_handle, VWErrorMessage* error_message_handle) noexcept;
  DLL_PUBLIC int VWWorkspaceParseDSJson(VWWorkspace* workspace_handle, const char* json_string, size_t length,
      VWMultiEx* output_handle, VWErrorMessage* error_message_handle) noexcept;

  DLL_PUBLIC VWExample* VWExampleCreate() noexcept;
  DLL_PUBLIC void VWExampleDelete(VWExample* example_handle) noexcept;

  DLL_PUBLIC VWMultiEx* VWMultiExCreate() noexcept;
  // If any examples are held in the container they will be deleted too.
  DLL_PUBLIC void VWMultiExDelete(VWMultiEx* example_handle) noexcept;
  DLL_PUBLIC size_t VWMultiGetLength(VWMultiEx* example_handle) noexcept;
  DLL_PUBLIC int VWMultiGetExample(
      VWMultiEx* example_handle, VWExample** examples, size_t index, VWErrorMessage* error_message_handle) noexcept;
  // Does not delete the contained examples.
  DLL_PUBLIC void VWMultiClear(VWMultiEx* example_handle) noexcept;
}
