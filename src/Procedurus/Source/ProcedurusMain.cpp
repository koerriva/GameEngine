/*******************************************************************************
* File:           ProcedurusMain.h
* Author:         Christian Alfons
* Date created:   2010-06-29
* Date modified:  2010-12-03
* Description:    The main file of the Procedurus application.
*******************************************************************************/

#include <iostream>
#include "Application/Application.h"

using std::cout;
using std::endl;

// Application variables
bool done = false;
Application *application;

// Exits the main loop
int StopExecution()
{
  // Set the application status to done
  done = true;

  // Don't close the window just yet
  return GL_FALSE;
}

// Key callback function
void HandleKey(int key, int action)
{
  // If a key was pressed
  if (action == GLFW_PRESS && key == GLFW_KEY_ESC)
  {
    StopExecution();
    return;
  }

  application->HandleKey(key, action);
}

// The main entry point for the application
int main(int argc, char *argv[])
{
  // Initialize GLFW
  if (!glfwInit())
  {
    // Print an error message
    cout << "glfwInit failed" << endl;

    return 1;
  }

  // Open a GLFW window
  if (!glfwOpenWindow(1280, 800, 8, 8, 8, 8, 32, 0, GLFW_WINDOW))
  {
    // Terminate GLFW
    glfwTerminate();

    // Print an error message
    cout << "glfwOpenWindow failed" << endl;

    return 2;
  }

  // Set the window title
  glfwSetWindowTitle("Procedurus");

  // Enable mouse cursor
  glfwEnable(GLFW_MOUSE_CURSOR);

  // Enable keyboard key repeat
  glfwEnable(GLFW_KEY_REPEAT);

  // Turn vsync off for benchmarking purposes
  glfwSwapInterval(0);

  // Set GLFW callback functions
  glfwSetWindowCloseCallback(StopExecution);
  glfwSetKeyCallback(HandleKey);

  // Create an application instance
  application = new Application();

  // Get the current time
  double timeLastFrame = glfwGetTime();

  // Start the main loop
  while (!done)
  {
    // Calculate the frame time
    const double currentTime = glfwGetTime();
    const double frameTime = currentTime - timeLastFrame;
    timeLastFrame = currentTime;

    // Update and render the application
    application->Update(frameTime);
    application->Render();

    // Print any OpenGL error flag value
    static GLenum lastError = GL_NO_ERROR;
    while (GLenum error = glGetError() != GL_NO_ERROR)
    {
      if (error != lastError)
      {
        cout << "OpenGL error: " << error << endl;
        lastError = error;
      }
    }
  }

  // Delete pointers here to deinitialize OpenGL stuff before terminating GLFW
  delete application;

  // Terminate GLFW
  glfwTerminate();

  return 0;
}
