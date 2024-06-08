class LoginFormComponent extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.shadowRoot.innerHTML = `
            <style>
                .login-form {
                    display: flex;
                    flex-direction: column;
                    gap: 1rem;
                    background: #f9fafb;
                    padding: 2rem;
                    border-radius: 0.5rem;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                }
                .login-form input {
                    padding: 0.5rem;
                    border: 1px solid #ccc;
                    border-radius: 0.25rem;
                }
                .login-form button {
                    padding: 0.75rem;
                    border: none;
                    border-radius: 0.25rem;
                    background: #1E40AF;
                    color: #fff;
                    cursor: pointer;
                    transition: background 0.3s ease;
                }
                .login-form button:hover {
                    background: #1b3798;
                }
                .fade-in {
                    opacity: 0;
                    transition: opacity 0.5s ease-in-out;
                }
                .fade-in.visible {
                    opacity: 1;
                }
                .highlight {
                    color: #1E40AF; /* Tailwind blue-800 */
                }
                .hidden {
                    display: none;
                }
            </style>
            <section class="bg-gray-50 p-4 rounded-lg shadow fade-in" id="login-section">
                <h2 class="text-2xl font-semibold mb-4"><span class="highlight">Login</span></h2>
                <form id="login-form">
                    <div class="mb-4">
                        <label for="username" class="block text-sm font-medium text-gray-700">Username</label>
                        <input type="text" id="username" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                    </div>
                    <div class="mb-4">
                        <label for="password" class="block text-sm font-medium text-gray-700">Password</label>
                        <input type="password" id="password" class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm">
                    </div>
                    <button type="submit" class="w-full py-2 px-4 bg-blue-500 text-white font-semibold rounded-md shadow hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500">Login</button>
                </form>
            </section>
        `;

        this.shadowRoot.querySelector('form').addEventListener('submit', this.handleLogin.bind(this));
    }

    handleLogin(event) {
        event.preventDefault();
        const username = this.shadowRoot.querySelector('#username').value;

        document.getElementById('user-name').textContent = username;
        document.getElementById('login-section').classList.add('hidden');
        document.getElementById('user-section').classList.remove('hidden');
        document.getElementById('notifications-section').classList.remove('hidden');
        document.getElementById('stats-section').classList.remove('hidden');
    }
}

customElements.define('login-form-component', LoginFormComponent);
