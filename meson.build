project('volkano', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3'])

glm_dep = dependency('glm', required:false)
if not glm_dep.found()
  glm_proj = subproject('glm')
  glm_dep = glm_proj.get_variable('glm_dep')
endif

glfw_dep = dependency('glfw', required:false)

glfw_proj = subproject('glfw')
glfw_dep = glfw_proj.get_variable('glfw_dep')

vulkan_dep = dependency('vulkan')
#vulkan_headers_dep = dependency('vulkan-headers', required:false)
#if not vulkan_headers_dep.found()
#  vulkan_headers_proj = subproject('vulkan-headers')
#  vulkan_headers_dep = vulkan_headers_proj.get_variable('vulkan_headers_dep')
#endif


exe = executable('vulkano', 'vulkano.cpp',
      dependencies: [glfw_dep, glm_dep, vulkan_dep ],
  install : true)

test('basic', exe)
