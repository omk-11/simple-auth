document.getElementById('auth-form').addEventListener('submit', async function(event) {
    event.preventDefault();
    
    const password = document.getElementById('password').value;
    
    const response = await fetch('http://localhost:8080/auth', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ password })
    });
  
    const result = await response.json();
    const message = result.success ? "Authentication successful!" : "Authentication failed.";
    document.getElementById('response').innerText = message;
  });
  