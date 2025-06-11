(function() {
    // Get saved theme or default to system preference
    const savedTheme = localStorage.getItem('theme');
    const systemTheme = window.matchMedia('(prefers-color-scheme: light)').matches ? 'light' : 'dark';
    const currentTheme = savedTheme || systemTheme;
    
    // Apply theme to document
    function setTheme(theme) {
        if (theme === 'auto') {
            document.documentElement.removeAttribute('data-theme');
        } else {
            document.documentElement.setAttribute('data-theme', theme);
        }
        localStorage.setItem('theme', theme);
        updateToggleIcon(theme);
    }
    
    // Update toggle button icon
    function updateToggleIcon(theme) {
        const toggle = document.querySelector('.theme-toggle .icon');
        if (toggle) {
            if (theme === 'light') {
                toggle.textContent = '🌙';
            } else {
                toggle.textContent = '☀️';
            }
        }
    }
    
    // Initialize theme
    setTheme(currentTheme);
    
    // Theme toggle functionality
    function toggleTheme() {
        const currentTheme = document.documentElement.getAttribute('data-theme');
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        setTheme(newTheme);
    }
    
    // Set up toggle button when DOM is ready
    document.addEventListener('DOMContentLoaded', function() {
        const toggleButton = document.querySelector('.theme-toggle');
        if (toggleButton) {
            toggleButton.addEventListener('click', toggleTheme);
            updateToggleIcon(currentTheme);
        }
    });
    
    // Listen for system theme changes
    window.matchMedia('(prefers-color-scheme: light)').addEventListener('change', function(e) {
        if (!localStorage.getItem('theme')) {
            setTheme('auto');
        }
    });
})();
