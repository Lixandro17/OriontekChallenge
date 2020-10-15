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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable


//Open browser
WebUI.openBrowser(null)
WebUI.navigateToUrl("https://www.amazon.com")
WebUI.comment('The user must be able to navigate to the Amazon site')

//Verify search bar existence
WebUI.verifyElementPresent(findTestObject('Object Repository/SearchBar/div_Todos'), 0, FailureHandling.STOP_ON_FAILURE)
WebUI.comment('Given that the user lands on the amazon page it must be able to confirm the existence of the search bar')

//Enter search criteria
WebUI.setText(findTestObject('SearchBar/input_field-keywords'), 'PS4')
WebUI.click(findTestObject('SearchButton/input_nav-input'))
WebUI.comment('The user must be able to enter a search criteria and filter by the specific criteria')

//Verfify specific item existence and click it
WebUI.verifyElementPresent(findTestObject('Object Repository/FilterItem/SpecificItem'), 0, FailureHandling.STOP_ON_FAILURE)
WebUI.click(findTestObject('Object Repository/FilterItem/SpecificItem'))
WebUI.comment('Once the filter result comes back the user must be able to confirm the existence of the specific item and by it click it')

//Add item to cart
WebUI.verifyElementPresent(findTestObject('Object Repository/SearchCriteria/input_nav-input'), 0, FailureHandling.STOP_ON_FAILURE)
WebUI.verifyElementPresent(findTestObject('Object Repository/SearchCriteria/SONY PlayStation 4 Slim 1TB Console-item'),0)
WebUI.verifyElementPresent(findTestObject('Object Repository/ItemAddtoCart/input_submit.add-to-cart'), 0, FailureHandling.STOP_ON_FAILURE)
WebUI.click(findTestObject('Object Repository/ItemAddtoCart/input_submit.add-to-cart', FailureHandling.STOP_ON_FAILURE))
WebUI.comment('The user must be able to access the specific item page and by it click on the Add to Cart button')

//Verify the item addition correctly
WebUI.verifyElementPresent(findTestObject('Object Repository/AddedItemConfirmation/AddedtoCart'), 0, FailureHandling.STOP_ON_FAILURE)
WebUI.comment('Given that the Add to Cart button was clicked the user must be able to confirm that the specific item was added to the cart correctly')
  
//Close browser
WebUI.closeBrowser()
WebUI.comment('Once the add to cart process is complete the browser must be close correctly')