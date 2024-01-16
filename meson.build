project('volkano', 'cpp',
  version : '0.1',
  default_options : ['warning_level=3'])

glfw_proj = subproject('glfw')
glfw_dep = glfw_proj.get_variable('glfw_dep')

exe = executable('vulkano', 'vulkano.cpp',
      dependencies: glfw_dep,
  install : true)

test('basic', exe)
