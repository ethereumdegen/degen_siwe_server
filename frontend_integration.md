# Frontend Integration Guide

This document provides detailed instructions for frontend engineers on how to integrate with the SIWE (Sign-In with Ethereum) authentication endpoints provided by the degen_siwe_server.

## Overview

The degen_siwe_server provides a complete Ethereum wallet-based authentication flow using the Sign-In with Ethereum standard. This allows users to authenticate using their Web3 wallets (like MetaMask, WalletConnect, etc.) without the need for traditional username/password credentials.

The authentication flow consists of two main steps:
1. Generating a challenge message that the user will sign with their wallet
2. Validating the signature to authenticate the user and create a session

## Authentication Endpoints

The server exposes two main endpoints for this authentication flow:

### 1. Generate Challenge (`/api/session/generate_challenge`)

This endpoint generates a unique challenge message that the user will need to sign with their Ethereum wallet.

#### Request

- **Method**: POST
- **URL**: `/api/session/generate_challenge`
- **Headers**:
  ```
  Content-Type: application/json
  ```
- **Body**:
  ```json
  {
    "public_address": "0xYourEthereumAddressHere"
  }
  ```

#### Response

- **Success Response** (200 OK):
  ```json
  {
    "success": true,
    "challenge": "Signing in to [service_name] as 0xyourethereumaddresshere at [timestamp]",
    "error": null
  }
  ```

- **Error Response** (400 Bad Request):
  ```json
  {
    "success": false,
    "challenge": null,
    "error": "Invalid public address"
  }
  ```

- **Error Response** (500 Internal Server Error):
  ```json
  {
    "success": false,
    "challenge": null,
    "error": "Database error"
  }
  ```

### 2. Validate Authentication (`/api/session/validate_auth`)

This endpoint validates the signed challenge and creates a session for the authenticated user.

#### Request

- **Method**: POST
- **URL**: `/api/session/validate_auth`
- **Headers**:
  ```
  Content-Type: application/json
  ```
- **Body**:
  ```json
  {
    "public_address": "0xYourEthereumAddressHere",
    "challenge": "The challenge message received from generate_challenge",
    "signature": "0xTheSignatureGeneratedByTheUsersWallet"
  }
  ```

#### Response

- **Success Response** (200 OK):
  ```json
  {
    "success": true,
    "data": {
      "public_address": "0xYourEthereumAddressHere",
      "session_token": "generated_session_token_here",
      "expires_at": 1740251911 // Unix timestamp when the session expires
    },
    "error": null
  }
  ```

- **Error Responses** (400/401/500):
  ```json
  {
    "success": false,
    "data": null,
    "error": "Error message" // Possible errors: "Invalid public address", "Invalid challenge", "No active challenge found", "Invalid signature", "Database error"
  }
  ```

## Integration Steps

### Step 1: Implementing the Authentication Flow

Here's a detailed implementation guide using modern JavaScript/TypeScript:

```javascript
// Authentication service
class Web3AuthService {
  constructor(apiBaseUrl) {
    this.apiBaseUrl = apiBaseUrl;
  }

  // Step 1: Generate a challenge for the user to sign
  async generateChallenge(publicAddress) {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/session/generate_challenge`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ public_address: publicAddress }),
      });

      const data = await response.json();
      
      if (!data.success) {
        throw new Error(data.error || 'Failed to generate challenge');
      }
      
      return data.challenge;
    } catch (error) {
      console.error('Error generating challenge:', error);
      throw error;
    }
  }

  // Step 2: Sign the challenge with the user's wallet
  async signChallenge(challenge, web3Provider) {
    try {
      const accounts = await web3Provider.request({ method: 'eth_requestAccounts' });
      const publicAddress = accounts[0];
      
      // Request signature from the wallet
      const signature = await web3Provider.request({
        method: 'personal_sign',
        params: [challenge, publicAddress],
      });
      
      return {
        publicAddress,
        challenge,
        signature,
      };
    } catch (error) {
      console.error('Error signing challenge:', error);
      throw error;
    }
  }

  // Step 3: Validate the signature and get a session token
  async validateAuthentication(publicAddress, challenge, signature) {
    try {
      const response = await fetch(`${this.apiBaseUrl}/api/session/validate_auth`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          public_address: publicAddress,
          challenge: challenge,
          signature: signature,
        }),
      });

      const data = await response.json();
      
      if (!data.success) {
        throw new Error(data.error || 'Authentication failed');
      }
      
      return data.data; // Contains session_token, public_address, and expires_at
    } catch (error) {
      console.error('Error validating authentication:', error);
      throw error;
    }
  }

  // Complete authentication flow
  async authenticate(web3Provider) {
    try {
      // Get the user's Ethereum address
      const accounts = await web3Provider.request({ method: 'eth_requestAccounts' });
      const publicAddress = accounts[0];
      
      // Generate challenge
      const challenge = await this.generateChallenge(publicAddress);
      
      // Sign the challenge
      const { signature } = await this.signChallenge(challenge, web3Provider);
      
      // Validate the authentication
      const sessionData = await this.validateAuthentication(publicAddress, challenge, signature);
      
      // Store the session token (in localStorage, sessionStorage, cookies, etc.)
      localStorage.setItem('sessionToken', sessionData.session_token);
      localStorage.setItem('publicAddress', sessionData.public_address);
      localStorage.setItem('sessionExpiry', sessionData.expires_at.toString());
      
      return sessionData;
    } catch (error) {
      console.error('Authentication flow failed:', error);
      throw error;
    }
  }

  // Check if user is authenticated
  isAuthenticated() {
    const sessionToken = localStorage.getItem('sessionToken');
    const sessionExpiry = localStorage.getItem('sessionExpiry');
    
    if (!sessionToken || !sessionExpiry) {
      return false;
    }
    
    // Check if session is expired
    const expiryTimestamp = parseInt(sessionExpiry, 10) * 1000; // Convert to milliseconds
    const currentTime = Date.now();
    
    return currentTime < expiryTimestamp;
  }

  // Logout function
  logout() {
    localStorage.removeItem('sessionToken');
    localStorage.removeItem('publicAddress');
    localStorage.removeItem('sessionExpiry');
  }
}
```

### Step 2: Using the Authentication Service in Your Application

Here's an example of how to integrate the authentication service into a React application:

```jsx
import React, { useState, useEffect } from 'react';
import Web3AuthService from './services/Web3AuthService';

// Initialize the auth service
const authService = new Web3AuthService('https://your-api-endpoint.com');

function LoginButton() {
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState(null);
  
  const connectWallet = async () => {
    setIsLoading(true);
    setError(null);
    
    try {
      // Check if MetaMask is installed
      if (!window.ethereum) {
        throw new Error('Please install MetaMask to use this feature');
      }
      
      // Authenticate with the wallet
      await authService.authenticate(window.ethereum);
      
      // Redirect or update UI based on successful login
      window.location.href = '/dashboard';
    } catch (error) {
      setError(error.message);
    } finally {
      setIsLoading(false);
    }
  };
  
  return (
    <div>
      <button 
        onClick={connectWallet} 
        disabled={isLoading}
      >
        {isLoading ? 'Connecting...' : 'Connect Wallet'}
      </button>
      
      {error && <p className="error">{error}</p>}
    </div>
  );
}

function App() {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  
  useEffect(() => {
    // Check authentication status when the component mounts
    setIsAuthenticated(authService.isAuthenticated());
  }, []);
  
  const handleLogout = () => {
    authService.logout();
    setIsAuthenticated(false);
    // Redirect to login page
    window.location.href = '/login';
  };
  
  return (
    <div className="App">
      <header>
        <h1>My Web3 App</h1>
        {isAuthenticated ? (
          <button onClick={handleLogout}>Disconnect Wallet</button>
        ) : (
          <LoginButton />
        )}
      </header>
      
      <main>
        {isAuthenticated ? (
          <p>Welcome, {localStorage.getItem('publicAddress')}</p>
        ) : (
          <p>Please connect your wallet to continue</p>
        )}
      </main>
    </div>
  );
}

export default App;
```

## Using with Different Web3 Providers

The examples above use MetaMask (window.ethereum), but you can adapt the code for other Web3 providers:

### WalletConnect Example

```javascript
import WalletConnectProvider from "@walletconnect/web3-provider";

// Setup WalletConnect Provider
const provider = new WalletConnectProvider({
  infuraId: "YOUR_INFURA_PROJECT_ID",
});

// Connect to WalletConnect
await provider.enable();

// Use the provider with our auth service
await authService.authenticate(provider);
```

### Web3Modal Example (supports multiple wallets)

```javascript
import Web3Modal from "web3modal";
import WalletConnectProvider from "@walletconnect/web3-provider";

const web3Modal = new Web3Modal({
  network: "mainnet",
  cacheProvider: true,
  providerOptions: {
    walletconnect: {
      package: WalletConnectProvider,
      options: {
        infuraId: "YOUR_INFURA_PROJECT_ID",
      },
    },
  },
});

// Open modal and get provider
const provider = await web3Modal.connect();

// Use the provider with our auth service
await authService.authenticate(provider);
```

## Best Practices

1. **Error Handling**: Always implement proper error handling to provide clear feedback to users when authentication fails.

2. **Session Management**: Store the session token securely and check its expiration regularly. Implement automatic logout when sessions expire.

3. **Progressive Enhancement**: Provide fallback options for users without Web3 wallets.

4. **Mobile Support**: Ensure your integration works well on mobile devices by testing with mobile wallet apps.

5. **Network Handling**: Consider handling different blockchain networks and informing users if they need to switch networks.

## Security Considerations

1. **HTTPS**: Always use HTTPS for API requests to ensure signatures and tokens are transmitted securely.

2. **XSS Protection**: Be careful about how you store the session token to avoid XSS vulnerabilities.

3. **Session Expiry**: Honor the expiration time provided by the server and implement proper session refresh mechanisms.

4. **Message Signing**: Always use the standard personal_sign method for signing messages, as it's the most widely supported.

## Troubleshooting

### Common Issues and Solutions

1. **"User denied message signature"**: The user rejected the signature request in their wallet. Provide clear instructions about why signing is necessary.

2. **Network Mismatch**: Ensure users are connected to the correct network required by your application.

3. **Invalid Signature Errors**: Double-check that you're passing the exact challenge string received from the server without any modifications.

4. **Session Expiration**: If users are unexpectedly logged out, check that you're correctly handling the session expiration timestamp.

For any other issues, please contact the API team.