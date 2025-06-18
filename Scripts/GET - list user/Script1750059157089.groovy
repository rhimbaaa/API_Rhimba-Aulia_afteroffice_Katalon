import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

// Kirim request GET list user
def response = WS.sendRequest(findTestObject('GET - list user'))

// Verifikasi status code = 200
WS.verifyResponseStatusCode(response, 200)

// Parse response JSON
def slurper = new groovy.json.JsonSlurper()
def result = slurper.parseText(response.getResponseBodyContent())

// Verifikasi email user ke-3
def email = result.data[2].email
assert email != null                         // Pastikan email tidak null
assert email instanceof String               // Pastikan tipe data String
assert email == "tobias.funke@reqres.in"     // Verifikasi nilai email

// Verifikasi first name user ke-3
def firstname = result.data[2].first_name
assert firstname != null                     // Pastikan tidak null
assert firstname instanceof String           // Pastikan tipe data String
assert firstname == "Tobias"                 // Verifikasi nilai nama depan

// Verifikasi ID user pertama
def id = result.data[0].id
assert id != null                            // Pastikan tidak null
assert id instanceof Integer                 // Pastikan tipe data Integer
assert id == 7                               // Verifikasi nilai ID