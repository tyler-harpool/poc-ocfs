class NotificationsComponent extends HTMLElement {
    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
    }

    connectedCallback() {
        this.render();
    }

    render() {
        this.shadowRoot.innerHTML = `
            <style>
                .notifications {
                    background-color: #f9fafb;
                    padding: 16px;
                    border-radius: 8px;
                    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
                }
                .notification-item {
                    padding: 8px 0;
                    border-bottom: 1px solid #e5e7eb;
                }
                .notification-item:last-child {
                    border-bottom: none;
                }
            </style>
            <div id="notifications-section" class="notifications hidden">
                <h2 class="text-2xl font-semibold mb-4">Recent Notifications</h2>
                <ul>
                    <li class="notification-item">New case <strong>Akira Harrison v. The State of Texas</strong> added.</li>
                    <li class="notification-item">Document <strong>Motion to Dismiss</strong> filed in case <strong>Elsie Torres Franco v. The State of Texas</strong>.</li>
                    <li class="notification-item">User <strong>Jane Doe</strong> granted access to <strong>Case Management</strong>.</li>
                </ul>
            </div>
        `;
    }
}

customElements.define('notifications-component', NotificationsComponent);
