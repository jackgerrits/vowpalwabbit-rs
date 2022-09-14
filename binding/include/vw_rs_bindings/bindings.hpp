#pragma once

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

// For operations which cannot fail under any circumstance (except out of memory) it is acceptable to omit the return code, and error holder.
// If it is an operation which can fail, it must return an error code and accept the error message parameter for filling with failure info.

extern "C"
{
  static const int VW_STATUS_SUCCESS = 0;
  static const int VW_STATUS_FAIL = 1;

  struct VWWorkspace;
  struct VWExample;
  struct VWErrorMessage;

  DLL_PUBLIC struct VWErrorMessage* VWErrorMessageCreate() noexcept;
  DLL_PUBLIC void VWErrorMessageDelete(struct VWErrorMessage* error_message_handle) noexcept;
  // If there was no error message set, a nullptr is returned.
  DLL_PUBLIC const char* VWErrorMessageGetValue(const struct VWErrorMessage* error_message_handle) noexcept;
  DLL_PUBLIC void VWErrorMessageClearValue(struct VWErrorMessage* error_message_handle) noexcept;

  DLL_PUBLIC int VWWorkspaceInitialize(const char* const* tokens, int count, struct VWWorkspace** output_handle,
      struct VWErrorMessage* error_message) noexcept;
  DLL_PUBLIC void VWWorkspaceDelete(struct VWWorkspace* workspace_handle) noexcept;

  DLL_PUBLIC int VWWorkspaceLearn(struct VWWorkspace* workspace_handle, struct VWExample* example_handle, struct VWErrorMessage* error_message_handle) noexcept;

  DLL_PUBLIC int VWWorkspaceGetPooledExample(struct VWWorkspace* workspace_handle, struct VWExample** output_handle, struct VWErrorMessage* error_message_handle) noexcept;
  DLL_PUBLIC int VWWorkspaceReturnPooledExample(struct VWWorkspace* workspace_handle, struct VWExample* example_handle, struct VWErrorMessage* error_message_handle) noexcept;

  DLL_PUBLIC struct VWExample* VWExampleCreate() noexcept;
  DLL_PUBLIC void VWExampleDelete(struct VWExample* example_handle) noexcept;
}

