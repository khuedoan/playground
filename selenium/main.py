from selenium import webdriver
from selenium.webdriver.common.by import By

driver = webdriver.Firefox()
driver.get("https://github.com/login")
driver.find_element(By.ID, "login_field").send_keys("exampleuser")
driver.find_element(By.ID, "password").send_keys("examplepassword")
