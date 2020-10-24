# CMake 最低版本号要求
cmake_minimum_required (VERSION 3.6)

# 项目信息
project (GameEngine)

# 编译器配置
set(CMAKE_CXX_STANDARD 17)

# 查找依赖库
find_package(OpenGL REQUIRED)

# 版本号.
set (GameEngine_VERSION_MAJOR 0)
set (GameEngine_VERSION_MINOR 1)

# 设置源文件目录
set (PROJECT_SOURCE_DIR src)

# 查找当前目录下的所有源文件
# 并将名称保存到 DIR_SRCS 变量
aux_source_directory(src DIR_SRCS)

# 包含目录
include_directories(${CMAKE_CURRENT_LIST_DIR}/include)
include_directories(ThirdLibrary/glad)

# 添加库
add_subdirectory(ThirdLibrary/glad)
add_subdirectory(ThirdLibrary/glfw)
add_subdirectory(ThirdLibrary/spdlog)

# 指定生成目标
add_executable(GameEngine src/main.cpp ${PROJECT_SOURCE_DIR})

target_link_libraries(GameEngine glad glfw spdlog)