document.addEventListener("DOMContentLoaded", () => {
  const loginForm = document.getElementById("loginForm");
  const forgotPasswordForm = document.getElementById("forgotPasswordForm");

  function displayMessage(message, type = "error") {
    const formMessage = document.getElementById("formMessage");
    if (formMessage) {
      formMessage.textContent = message;
      formMessage.className = `form-message ${type}`;
      formMessage.style.display = "block";
    }
  }

  if (loginForm) {
    loginForm.addEventListener("submit", async (event) => {
      event.preventDefault();
      displayMessage("", "success"); // Clear previous messages

      const email = document.getElementById("email").value;
      const password = document.getElementById("password").value;

      try {
        const response = await fetch("/auth/login", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          credentials: "include", // Include cookies in request
          body: JSON.stringify({ email, password }),
        });

        if (response.ok) {
          const data = await response.json();
          // JWT is now stored in HTTP-only cookie by backend
          // No need to store in localStorage (security improvement)
          if (data.success) {
            window.location.href = "/index.html"; // Redirect to dashboard
          } else {
            displayMessage(data.message || "Login failed", "error");
          }
        } else {
          const errorData = await response.json();
          displayMessage(errorData.message || "Login failed", "error");
        }
      } catch (error) {
        console.error("Error during login:", error);
        displayMessage(
          "An error occurred during login. Please try again.",
          "error",
        );
      }
    });
  }

  if (forgotPasswordForm) {
    forgotPasswordForm.addEventListener("submit", async (event) => {
      event.preventDefault();
      displayMessage("", "success"); // Clear previous messages

      const email = document.getElementById("email").value;

      try {
        const response = await fetch("/auth/forgot-password", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ email }),
        });

        if (response.ok) {
          displayMessage(
            "If an account with that email exists, a password reset link has been sent.",
            "success",
          );
        } else {
          const errorData = await response.json();
          displayMessage(
            `Password reset failed: ${errorData.message}`,
            "error",
          );
        }
      } catch (error) {
        console.error("Error during forgot password:", error);
        displayMessage("An error occurred. Please try again.", "error");
      }
    });
  }
});
