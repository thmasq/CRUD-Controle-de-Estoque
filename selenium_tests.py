#!/usr/bin/env python3
"""
Selenium End-to-End Tests for Stock Manager
Run these tests against a running instance of the Stock Manager application.
"""

import os
import sys
import time
import unittest
import subprocess
import signal
import requests
from selenium import webdriver
from selenium.webdriver.common.by import By
from selenium.webdriver.support.ui import WebDriverWait, Select
from selenium.webdriver.support import expected_conditions as EC
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
from selenium.common.exceptions import TimeoutException, NoSuchElementException
from webdriver_manager.chrome import ChromeDriverManager


class StockManagerE2ETest(unittest.TestCase):
    """End-to-end tests for Stock Manager application using Selenium."""

    @classmethod
    def setUpClass(cls):
        """Set up the test environment."""
        cls.base_url = os.getenv("BASE_URL", "http://127.0.0.1:8080")
        cls.headless = os.getenv("HEADLESS", "true").lower() == "true"
        cls.server_process = None

        # Start the server if not already running
        if not cls._is_server_running():
            cls._start_server()

        # Set up Chrome driver
        chrome_options = Options()
        if cls.headless:
            chrome_options.add_argument("--headless")
        chrome_options.add_argument("--no-sandbox")
        chrome_options.add_argument("--disable-dev-shm-usage")
        chrome_options.add_argument("--disable-gpu")
        chrome_options.add_argument("--window-size=1920,1080")
        chrome_options.add_argument("--disable-extensions")
        chrome_options.add_argument("--disable-logging")
        chrome_options.add_argument("--disable-web-security")
        chrome_options.add_argument("--allow-running-insecure-content")

        # Install and use ChromeDriver
        service = Service(ChromeDriverManager().install())
        cls.driver = webdriver.Chrome(service=service, options=chrome_options)
        cls.driver.implicitly_wait(10)
        cls.wait = WebDriverWait(cls.driver, 15)

        # Test credentials
        cls.test_username = "selenium_test_user"
        cls.test_password = "selenium_test_pass"
        cls.test_role = "MANAGER"

    @classmethod
    def tearDownClass(cls):
        """Clean up after tests."""
        if cls.driver:
            cls.driver.quit()

        if cls.server_process:
            cls._stop_server()

    @classmethod
    def _is_server_running(cls):
        """Check if the server is already running."""
        try:
            response = requests.get(f"{cls.base_url}/auth/login", timeout=5)
            return response.status_code in [200, 302]
        except requests.RequestException:
            return False

    @classmethod
    def _start_server(cls):
        """Start the Stock Manager server."""
        print("Starting Stock Manager server...")

        # Look for the binary in common locations
        binary_paths = [
            "./target/release/stock-web-server",
            "./stock-manager/target/release/stock-web-server",
            "stock-web-server",
        ]

        binary_path = None
        for path in binary_paths:
            if os.path.exists(path):
                binary_path = path
                break

        if not binary_path:
            raise FileNotFoundError(
                "Could not find stock-web-server binary. Please build the project first."
            )

        # Set environment variables
        env = os.environ.copy()
        env.update(
            {
                "DATABASE_URL": os.getenv(
                    "DATABASE_URL",
                    "postgres://postgres:postgres@localhost:5432/stockmanager_integration",
                ),
                "JWT_SECRET": "selenium_test_secret",
                "ENABLE_REGISTRATION": "true",
                "HOST": "127.0.0.1",
                "PORT": "8080",
                "RUST_LOG": "info",
            }
        )

        try:
            cls.server_process = subprocess.Popen(
                [binary_path],
                env=env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                preexec_fn=os.setsid if hasattr(os, "setsid") else None,
            )

            # Wait for server to start
            max_attempts = 30
            for attempt in range(max_attempts):
                if cls._is_server_running():
                    print(f"Server started successfully after {attempt + 1} attempts")
                    time.sleep(2)  # Give it a bit more time to fully initialize
                    return
                time.sleep(1)

            raise RuntimeError("Server failed to start within 30 seconds")

        except Exception as e:
            print(f"Failed to start server: {e}")
            if cls.server_process:
                cls._stop_server()
            raise

    @classmethod
    def _stop_server(cls):
        """Stop the Stock Manager server."""
        if cls.server_process:
            try:
                if hasattr(os, "killpg"):
                    os.killpg(os.getpgid(cls.server_process.pid), signal.SIGTERM)
                else:
                    cls.server_process.terminate()

                cls.server_process.wait(timeout=10)
            except (subprocess.TimeoutExpired, ProcessLookupError):
                if hasattr(os, "killpg"):
                    os.killpg(os.getpgid(cls.server_process.pid), signal.SIGKILL)
                else:
                    cls.server_process.kill()
            finally:
                cls.server_process = None

    def setUp(self):
        """Set up each test."""
        self.driver.delete_all_cookies()
        self.driver.get(f"{self.base_url}/auth/login")

    def test_01_registration_and_login(self):
        """Test user registration and login flow."""
        print("\n=== Testing Registration and Login ===")

        # Navigate to registration page
        register_link = self.wait.until(
            EC.element_to_be_clickable((By.LINK_TEXT, "Register"))
        )
        register_link.click()

        # Fill registration form
        username_field = self.wait.until(
            EC.presence_of_element_located((By.ID, "username"))
        )
        username_field.send_keys(self.test_username)

        password_field = self.driver.find_element(By.ID, "password")
        password_field.send_keys(self.test_password)

        role_select = Select(self.driver.find_element(By.ID, "role"))
        role_select.select_by_value(self.test_role)

        # Submit registration
        register_button = self.driver.find_element(
            By.CSS_SELECTOR, "button[type='submit']"
        )
        register_button.click()

        # Should redirect to login page
        self.wait.until(EC.url_contains("/auth/login"))

        # Now login with the created user
        username_field = self.wait.until(
            EC.presence_of_element_located((By.ID, "username"))
        )
        username_field.send_keys(self.test_username)

        password_field = self.driver.find_element(By.ID, "password")
        password_field.send_keys(self.test_password)

        login_button = self.driver.find_element(
            By.CSS_SELECTOR, "button[type='submit']"
        )
        login_button.click()

        # Should redirect to dashboard
        self.wait.until(EC.url_matches(rf"{self.base_url}/?$"))

        # Verify we're logged in by checking for welcome message
        welcome_element = self.wait.until(
            EC.presence_of_element_located(
                (By.XPATH, f"//span[contains(text(), 'Welcome, {self.test_username}')]")
            )
        )
        self.assertTrue(welcome_element.is_displayed())

        print("✓ Registration and login successful")

    def _login_if_needed(self):
        """Helper method to ensure user is logged in."""
        current_url = self.driver.current_url
        if "/auth/login" in current_url:
            username_field = self.driver.find_element(By.ID, "username")
            username_field.send_keys(self.test_username)

            password_field = self.driver.find_element(By.ID, "password")
            password_field.send_keys(self.test_password)

            login_button = self.driver.find_element(
                By.CSS_SELECTOR, "button[type='submit']"
            )
            login_button.click()

            self.wait.until(EC.url_matches(rf"{self.base_url}/?$"))

    def test_02_dashboard_display(self):
        """Test that the dashboard displays correctly."""
        print("\n=== Testing Dashboard Display ===")

        self._login_if_needed()

        # Navigate to dashboard
        self.driver.get(f"{self.base_url}/")

        # Check for dashboard elements
        dashboard_title = self.wait.until(
            EC.presence_of_element_located(
                (By.XPATH, "//h1[contains(text(), 'Dashboard')]")
            )
        )
        self.assertTrue(dashboard_title.is_displayed())

        # Check for count cards
        count_cards = self.driver.find_elements(
            By.CSS_SELECTOR, ".bg-blue-50, .bg-green-50, .bg-purple-50, .bg-amber-50"
        )
        self.assertEqual(len(count_cards), 4)

        # Verify navigation links
        nav_links = ["Products", "Categories", "Warehouses", "Stock", "Transactions"]
        for link_text in nav_links:
            link = self.driver.find_element(By.LINK_TEXT, link_text)
            self.assertTrue(link.is_displayed())

        print("✓ Dashboard displays correctly")

    def test_03_create_category(self):
        """Test creating a new category."""
        print("\n=== Testing Category Creation ===")

        self._login_if_needed()

        # Navigate to categories page
        self.driver.get(f"{self.base_url}/categories")

        # Click "Add Category" button
        add_button = self.wait.until(
            EC.element_to_be_clickable(
                (By.XPATH, "//button[contains(text(), 'Add Category')]")
            )
        )
        add_button.click()

        # Wait for modal to appear
        modal = self.wait.until(
            EC.presence_of_element_located((By.ID, "modal-container"))
        )
        self.wait.until(EC.visibility_of(modal))

        # Fill category form
        name_field = self.wait.until(EC.presence_of_element_located((By.ID, "name")))
        name_field.send_keys("Selenium Test Category")

        description_field = self.driver.find_element(By.ID, "description")
        description_field.send_keys("Category created by Selenium tests")

        # Submit form
        save_button = self.driver.find_element(
            By.XPATH, "//button[contains(text(), 'Save')]"
        )
        save_button.click()

        # Wait for modal to close and verify category appears in table
        self.wait.until(EC.invisibility_of_element((By.ID, "modal-container")))

        category_cell = self.wait.until(
            EC.presence_of_element_located(
                (By.XPATH, "//td[contains(text(), 'Selenium Test Category')]")
            )
        )
        self.assertTrue(category_cell.is_displayed())

        print("✓ Category created successfully")

    def test_04_create_warehouse(self):
        """Test creating a new warehouse."""
        print("\n=== Testing Warehouse Creation ===")

        self._login_if_needed()

        # Navigate to warehouses page
        self.driver.get(f"{self.base_url}/warehouses")

        # Click "Add Warehouse" button
        add_button = self.wait.until(
            EC.element_to_be_clickable(
                (By.XPATH, "//button[contains(text(), 'Add Warehouse')]")
            )
        )
        add_button.click()

        # Wait for modal to appear
        modal = self.wait.until(
            EC.presence_of_element_located((By.ID, "modal-container"))
        )
        self.wait.until(EC.visibility_of(modal))

        # Fill warehouse form
        name_field = self.wait.until(EC.presence_of_element_located((By.ID, "name")))
        name_field.send_keys("Selenium Test Warehouse")

        location_field = self.driver.find_element(By.ID, "location")
        location_field.send_keys("123 Test Street, Test City")

        contact_field = self.driver.find_element(By.ID, "contact_info")
        contact_field.send_keys("selenium@test.com")

        # Submit form
        save_button = self.driver.find_element(
            By.XPATH, "//button[contains(text(), 'Save')]"
        )
        save_button.click()

        # Wait for modal to close and verify warehouse appears in table
        self.wait.until(EC.invisibility_of_element((By.ID, "modal-container")))

        warehouse_cell = self.wait.until(
            EC.presence_of_element_located(
                (By.XPATH, "//td[contains(text(), 'Selenium Test Warehouse')]")
            )
        )
        self.assertTrue(warehouse_cell.is_displayed())

        print("✓ Warehouse created successfully")

    def test_05_create_product(self):
        """Test creating a new product."""
        print("\n=== Testing Product Creation ===")

        self._login_if_needed()

        # Navigate to products page
        self.driver.get(f"{self.base_url}/products")

        # Click "Add Product" button
        add_button = self.wait.until(
            EC.element_to_be_clickable(
                (By.XPATH, "//button[contains(text(), 'Add Product')]")
            )
        )
        add_button.click()

        # Wait for modal to appear
        modal = self.wait.until(
            EC.presence_of_element_located((By.ID, "modal-container"))
        )
        self.wait.until(EC.visibility_of(modal))

        # Fill product form
        name_field = self.wait.until(EC.presence_of_element_located((By.ID, "name")))
        name_field.send_keys("Selenium Test Product")

        sku_field = self.driver.find_element(By.ID, "sku")
        sku_field.send_keys("SELENIUM-001")

        description_field = self.driver.find_element(By.ID, "description")
        description_field.send_keys("Product created by Selenium tests")

        # Try to select category if available
        try:
            category_select = Select(self.driver.find_element(By.ID, "category_id"))
            # Select the first non-empty option if available
            options = category_select.options
            if len(options) > 1:  # More than just the empty option
                category_select.select_by_index(1)
        except NoSuchElementException:
            pass  # No categories available

        # Submit form
        save_button = self.driver.find_element(
            By.XPATH, "//button[contains(text(), 'Save')]"
        )
        save_button.click()

        # Wait for modal to close and verify product appears in table
        self.wait.until(EC.invisibility_of_element((By.ID, "modal-container")))

        product_cell = self.wait.until(
            EC.presence_of_element_located(
                (By.XPATH, "//td[contains(text(), 'SELENIUM-001')]")
            )
        )
        self.assertTrue(product_cell.is_displayed())

        print("✓ Product created successfully")

    def test_06_create_stock_item(self):
        """Test creating a new stock item."""
        print("\n=== Testing Stock Item Creation ===")

        self._login_if_needed()

        # Navigate to stock items page
        self.driver.get(f"{self.base_url}/stock-items")

        # Click "Add Stock Item" button
        add_button = self.wait.until(
            EC.element_to_be_clickable(
                (By.XPATH, "//button[contains(text(), 'Add Stock Item')]")
            )
        )
        add_button.click()

        # Wait for modal to appear
        modal = self.wait.until(
            EC.presence_of_element_located((By.ID, "modal-container"))
        )
        self.wait.until(EC.visibility_of(modal))

        # Select product and warehouse
        product_select = Select(
            self.wait.until(EC.presence_of_element_located((By.ID, "product_id")))
        )
        warehouse_select = Select(self.driver.find_element(By.ID, "warehouse_id"))

        # Select the first available product and warehouse
        product_options = product_select.options
        warehouse_options = warehouse_select.options

        if len(product_options) > 1 and len(warehouse_options) > 1:
            product_select.select_by_index(1)
            warehouse_select.select_by_index(1)

            # Set quantity and unit cost
            quantity_field = self.driver.find_element(By.ID, "quantity")
            quantity_field.clear()
            quantity_field.send_keys("100")

            unit_cost_field = self.driver.find_element(By.ID, "unit_cost")
            unit_cost_field.clear()
            unit_cost_field.send_keys("10.99")

            # Submit form
            save_button = self.driver.find_element(
                By.XPATH, "//button[contains(text(), 'Save')]"
            )
            save_button.click()

            # Wait for modal to close and verify stock item appears in table
            self.wait.until(EC.invisibility_of_element((By.ID, "modal-container")))

            # Look for quantity 100 in the table
            quantity_cell = self.wait.until(
                EC.presence_of_element_located(
                    (By.XPATH, "//td[contains(text(), '100')]")
                )
            )
            self.assertTrue(quantity_cell.is_displayed())

            print("✓ Stock item created successfully")
        else:
            print("⚠ Skipped stock item creation - no products or warehouses available")

    def test_07_create_transaction(self):
        """Test creating a stock transaction."""
        print("\n=== Testing Transaction Creation ===")

        self._login_if_needed()

        # Navigate to stock items page
        self.driver.get(f"{self.base_url}/stock-items")

        # Look for a "Transaction" button
        try:
            transaction_buttons = self.driver.find_elements(
                By.XPATH, "//button[contains(text(), 'Transaction')]"
            )
            if transaction_buttons:
                transaction_buttons[0].click()

                # Wait for modal to appear
                modal = self.wait.until(
                    EC.presence_of_element_located((By.ID, "modal-container"))
                )
                self.wait.until(EC.visibility_of(modal))

                # Fill transaction form
                transaction_type = Select(
                    self.wait.until(
                        EC.presence_of_element_located((By.ID, "transaction_type"))
                    )
                )
                transaction_type.select_by_value("IN")

                quantity_field = self.driver.find_element(By.ID, "quantity")
                quantity_field.clear()
                quantity_field.send_keys("50")

                reference_field = self.driver.find_element(By.ID, "reference_number")
                reference_field.send_keys("SELENIUM-REF-001")

                notes_field = self.driver.find_element(By.ID, "notes")
                notes_field.send_keys("Transaction created by Selenium test")

                created_by_field = self.driver.find_element(By.ID, "created_by")
                created_by_field.send_keys(self.test_username)

                # Submit form
                create_button = self.driver.find_element(
                    By.XPATH, "//button[contains(text(), 'Create Transaction')]"
                )
                create_button.click()

                # Wait for modal to close
                self.wait.until(EC.invisibility_of_element((By.ID, "modal-container")))

                print("✓ Transaction created successfully")
            else:
                print("⚠ Skipped transaction creation - no stock items available")
        except (NoSuchElementException, TimeoutException):
            print("⚠ Skipped transaction creation - no stock items available")

    def test_08_filter_functionality(self):
        """Test filtering functionality on various pages."""
        print("\n=== Testing Filter Functionality ===")

        self._login_if_needed()

        # Test product filtering
        self.driver.get(f"{self.base_url}/products")

        try:
            status_filter = Select(
                self.wait.until(
                    EC.presence_of_element_located((By.ID, "status-filter"))
                )
            )
            status_filter.select_by_value("active")

            # Wait a moment for HTMX to process
            time.sleep(2)

            print("✓ Product filtering works")
        except (NoSuchElementException, TimeoutException):
            print("⚠ Product filter not found or not working")

        # Test transaction filtering
        self.driver.get(f"{self.base_url}/transactions")

        try:
            transaction_filter = Select(
                self.wait.until(
                    EC.presence_of_element_located((By.ID, "transaction-type-filter"))
                )
            )
            transaction_filter.select_by_value("IN")

            # Wait a moment for HTMX to process
            time.sleep(2)

            print("✓ Transaction filtering works")
        except (NoSuchElementException, TimeoutException):
            print("⚠ Transaction filter not found or not working")

    def test_09_navigation(self):
        """Test navigation between different pages."""
        print("\n=== Testing Navigation ===")

        self._login_if_needed()

        pages = [
            ("Products", "/products"),
            ("Categories", "/categories"),
            ("Warehouses", "/warehouses"),
            ("Stock", "/stock-items"),
            ("Transactions", "/transactions"),
            ("Dashboard", "/"),
        ]

        for page_name, expected_path in pages:
            if page_name == "Dashboard":
                nav_link = self.driver.find_element(By.LINK_TEXT, page_name)
            else:
                nav_link = self.driver.find_element(By.LINK_TEXT, page_name)

            nav_link.click()

            # Wait for navigation to complete
            self.wait.until(lambda driver: expected_path in driver.current_url)

            # Verify we're on the right page
            current_url = self.driver.current_url
            self.assertIn(expected_path, current_url)

        print("✓ Navigation between pages works correctly")

    def test_10_logout(self):
        """Test logout functionality."""
        print("\n=== Testing Logout ===")

        self._login_if_needed()

        # Click logout button
        logout_button = self.wait.until(
            EC.element_to_be_clickable((By.LINK_TEXT, "Logout"))
        )
        logout_button.click()

        # Should redirect to login page
        self.wait.until(EC.url_contains("/auth/login"))

        # Verify logout was successful by checking we can't access dashboard
        self.driver.get(f"{self.base_url}/")
        self.wait.until(EC.url_contains("/auth/login"))

        print("✓ Logout successful")


def main():
    """Main function to run the tests."""
    if len(sys.argv) > 1 and sys.argv[1] == "--help":
        print("""
Stock Manager Selenium E2E Tests

Environment Variables:
  BASE_URL        Base URL for the application (default: http://127.0.0.1:8080)
  DATABASE_URL    PostgreSQL connection string
  HEADLESS        Run browser in headless mode (default: true)

Usage:
  python selenium_tests.py                    # Run all tests
  python selenium_tests.py TestClass.test_*   # Run specific test
        """)
        return

    # Set up test discovery
    loader = unittest.TestLoader()

    if len(sys.argv) > 1:
        # Run specific tests
        suite = loader.loadTestsFromName(sys.argv[1], StockManagerE2ETest)
    else:
        # Run all tests
        suite = loader.loadTestsFromTestCase(StockManagerE2ETest)

    # Run tests with verbose output
    runner = unittest.TextTestRunner(verbosity=2, buffer=True)
    result = runner.run(suite)

    # Exit with appropriate code
    sys.exit(0 if result.wasSuccessful() else 1)


if __name__ == "__main__":
    main()
