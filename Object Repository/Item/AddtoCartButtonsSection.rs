<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>AddtoCartButtonsSection</name>
   <tag></tag>
   <elementGuidId>8a9fa738-36a9-4278-a9c8-d95784d57eaa</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>#rightCol</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='rightCol']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>rightCol</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>rightCol rightCol-bbcxoverride</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>


















































Share

(function(f) {f(window.P._namespace(&quot;DetailPageTellAFriendTemplates&quot;));}(function(P) {
P.when('A','ready').execute('swfMailTo', function(A){
var $ = A.$;
function collectMetrics( placementId ) {
A.ajax('/gp/aw/social/swf/metrics.html/ref=cm_sw_cl_em_'+ placementId, {
method: 'post',
params: {
id: 'mailtoDp',
result: 'mailto share triggered'
}
});
}
$('#swfMailTo,#swfImage').click( function(){
collectMetrics('dp_56hIFbDCKQQJ0');
});
});
}));

























(function(f) {f(window.P._namespace(&quot;DetailPageTellAFriendTemplates&quot;));}(function(P) {
P.when('A', 'jQuery').execute('tellAFriendBox', function(A,$) {
var taf = $('#tell-a-friend');
taf.children(&quot;[data-height]&quot;).click(function() {
var height = $(this).attr('data-height');
window.open(this.href, '_blank', 'location=yes,width=700,height=' + height);
return false;
});
});
}));


















































































































































































    
        
        
        
            
            





























































    
        
        
        
            
            

































































































P.when(&quot;A&quot;, &quot;a-expander&quot;, &quot;ready&quot;).execute(function(A, expander) {
A.on(&quot;a:accordion:buybox-accordion:select&quot;, function(data) {
// Change active accordion header price color to red
A.$(&quot;#buyBoxAccordion&quot;).find(&quot;.accordion-header span.header-price&quot;).
removeClass(&quot;a-color-price&quot;).addClass(&quot;a-color-secondary&quot;);
A.$(data.selectedRow.$row).find(&quot;.accordion-header span.header-price&quot;).
removeClass(&quot;a-color-secondary&quot;).addClass(&quot;a-color-price&quot;);
// Change active accordion header prime logo to opaque
A.$(&quot;#buyBoxAccordion&quot;).find(&quot;.accordion-header i.header-prime-logo&quot;).
addClass(&quot;opacity-50&quot;);
A.$(data.selectedRow.$row).find(&quot;.accordion-header i.header-prime-logo&quot;).
removeClass(&quot;opacity-50&quot;);
//initialize accordion expander
expander.initializeExpanders();
});
// Record metrics for clicking usedAccordionRow
A.on(&quot;a:accordion:buybox-accordion:usedAccordionRow:select&quot;, function(data) {
var ue = window.ue;
if (ue &amp;&amp; typeof ue.count === 'function') {
ue.count('dpOffers:buybox:mobile:usedAccordionOpen', (ue.count('dpOffers:buybox:mobile:usedAccordionOpen') || 0) + 1);
}
});
});





































































































































    
    
        
    



    
    
        
    
    






   
       Buy new:
   
   
        
        
        
              
              $389.99
              
        
   

    
    

    
    
































    + $42.70
 shipping

































































    




                Arrives: 


Oct 20 - Nov 2














var LUXQuantityRefreshEnabled = false;
P.when(&quot;LUXDPQuantityRefresh&quot;).execute(function() {
LUXQuantityRefreshEnabled = true;
});
P.when(&quot;A&quot;, &quot;jQuery&quot;).execute(function(A, $) {
$(&quot;#quantity&quot;).live(&quot;change&quot;, function (event) {
// Let LocationUX handle quantity refreshes if it's enabled.
if (LUXQuantityRefreshEnabled) {
return;
}
if (event.updateFTOnQuantityChange) {
return;
}
event.updateFTOnQuantityChange = 1;
var quantity = $(this).val();
var asin = $(&quot;#ftSelectAsin&quot;).val();
var merchantId = $(&quot;#ftSelectMerchant&quot;).val();
if (!asin || !merchantId) {
return;
}
var params = [];
params.push(&quot;asin=&quot; + asin);
params.push(&quot;quantity=&quot; + quantity);
params.push(&quot;merchantId=&quot; + merchantId);
$.ajax({
type: &quot;POST&quot;,
url: &quot;/gp/product/features/dp-fast-track/udp-ajax-handler/get-quantity-update-message.html?ie=UTF8&quot;,
contentType: 'application/x-www-form-urlencoded;charset=utf-8',
data: params.join('&amp;'),
dataType: &quot;html&quot;,
success: function(objResponse) {
if (objResponse != null &amp;&amp; objResponse != &quot;&quot;) {
$(&quot;#fast-track-message&quot;).replaceWith(objResponse);
}
}
});
return;
});
});









    










Ships from:
Amazon




Sold by:
TrulyTheBest




























  
      
      
      
          
      
  


 

    

     
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
         
             

             
             

              



    
            
                
                




            

            
            

            
            

            
            
            
            
            
            
            

     









































    
        
        
        
            
                
            
            












































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































































Qty:




1







2







3







4







5







6







7







8







9







10







11







12







13







14







15







16







17







18







19







20







21







22







23







24







25







26







27







28










Qty:1













































































































































































































































































































































































































































































































































































































































































































































































































































































Add to Cart










(function(f) {f(window.P._namespace(&quot;DetailPageBuyBoxTemplate&quot;));}(function(P) {
P.now().execute('dp-mark-atc',function(){
if (typeof window.markFeatureRender === 'function') {
window.markFeatureRender('atc',{isInteractive:false});
}
});
}));









































































{&quot;heroName&quot;:&quot;&quot;}
[&quot;addServices&quot;]


































































































{&quot;turboWeblab&quot;:&quot;RCX_CHECKOUT_TURBO_DESKTOP_NONPRIME_87784&quot;,&quot;strings&quot;:{&quot;TURBO_CHECKOUT_HEADER&quot;:&quot;Buy now: SONY PlayStation 4 Slim 1TB Console, Light &amp; Slim PS4 System, 1TB Hard Drive, All the Greatest Games, TV, Music &amp; More&quot;,&quot;TURBO_LOADING_TEXT&quot;:&quot;Loading your order summary&quot;},&quot;inputs&quot;:{&quot;a&quot;:&quot;B077QT6K94&quot;,&quot;quantity&quot;:&quot;1&quot;,&quot;oid&quot;:&quot;&quot;,&quot;addressId&quot;:&quot;&quot;},&quot;configurations&quot;:{&quot;isSignInEnabled&quot;:true,&quot;initiateSelector&quot;:&quot;#buy-now-button&quot;,&quot;prefetchEnabled&quot;:true},&quot;buttonID&quot;:&quot;buy-now&quot;,&quot;eligibility&quot;:{&quot;isEligible&quot;:false},&quot;turboWeblabTreatment&quot;:&quot;T2&quot;,&quot;timeout&quot;:&quot;5000&quot;}


(function(f) {f(window.P._namespace(&quot;TurboClientDetailPage&quot;));}(function(P) {
P.when('cf').execute(function executeTurboAssetsLoadTriggerEvent() {
P.now('turbo-checkout-assets-load-trigger').execute(function(assetsLoadTrigger) {
if (assetsLoadTrigger) {
logTurboCounter(&quot;AssetTriggerDedupe&quot;);
return;
}
try {
P.declare('turbo-checkout-assets-load-trigger', true);
logTurboCounter('AssetTrigger');
} catch (e) {
logTurboCounter('AssetTriggerException');
}
});
function logTurboCounter(name) {
var counter = 'turboCheckout' + name;
if (window.ue &amp;&amp; window.ue.count) {
window.ue.count(counter, 1);
}
}
});
}));




















Buy Now




































































































Secure transaction








Your transaction is secure


We work hard to protect your security and privacy. Our payment security system encrypts your information during transmission. We don’t share your credit card details with third-party sellers, and we don’t sell your information to others.

Learn more
























































































































































Add gift options























































































































































































































































































































































 






















































































































Deliver to Dominican Republic


















































































































        
    


























    
 


































































































    




    
    
        
    
    





   
       Buy used:
   
   
        
        
        
              
              $318.96
              
        
   
   


















  
      
      
      
          
      
  


 

    

     
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
         
             

             
             

              



    
            
                
                




            

            
            

            
            

            
            
            
            
            
            
            

     

















































































































Used: Very Good


 | Details



Sold by



TechGadgets Inc.








Condition:
Used: Very Good







































































































































































Deliver to Dominican Republic


















































































































Add to Cart









































































































































































































    
 

























































































        
    























































































































































 































































































































































  
      
      
      
          
      
  


 

    

     
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
              
         
             

             
             

              



    
            
                
                




            

            
            

            
            

            
            
            
            
            
            
            

     



























































Add to List















Added to












Unable to add item to List. Please try again.


















Sorry, there was a problem.


There was an error retrieving your Wish Lists. Please try again.










Sorry, there was a problem.


List unavailable.





{&quot;showInlineLink&quot;:false,&quot;hzPopover&quot;:true,&quot;wishlistButtonId&quot;:&quot;add-to-wishlist-button&quot;,&quot;dropDownHtml&quot;:&quot;&quot;,&quot;inlineJsFix&quot;:true,&quot;wishlistButtonSubmitId&quot;:&quot;add-to-wishlist-button-submit&quot;,&quot;maxAjaxFailureCount&quot;:&quot;3&quot;,&quot;asin&quot;:&quot;B077QT6K94&quot;}







{&quot;formId&quot;:&quot;addToCart&quot;,&quot;showWishListDropDown&quot;:false,&quot;wishlistPopoverWidth&quot;:206,&quot;isAddToWishListDropDownAuiEnabled&quot;:true,&quot;showPopover&quot;:false}



(function(f) {f(window.P._namespace(&quot;list-CF-register-js&quot;));}(function(P) {
&quot;use strict&quot;;
window.P.now('atwl-cf').execute(function (module) {
var isRegistered = (typeof module !== 'undefined');
if (!isRegistered) {
window.P.register('atwl-cf');
}
});
}));














.registry-button-width {
width:100%;
margin-left: ;
margin-right: ;
}
.add-to-baby-button-spacing-bottom {
margin-bottom: 0;
}


    
 
















































































































































































        
    














































































































































































































































































































































P.when(&quot;A&quot;, &quot;load&quot;).execute(&quot;aod-assets-loaded&quot;, function(A){
function logAssetsNotLoaded() {
if (window.ueLogError) {
var customError = { message: 'Failed to load AOD assets for WDG: video_games_display_on_website, Device: web' };
var additionalInfo = {
logLevel : 'ERROR',
attribution : 'aod_assets_not_loaded'
};
ueLogError (customError, additionalInfo);
}
if (window.ue &amp;&amp; window.ue.count) {
window.ue.count(&quot;aod-assets-not-loaded&quot;, 1);
}
}
function verifyAssetsLoaded() {
var assetsLoadedPageState = A.state('aod:assetsLoaded');
var logAssetsNotLoadedState = A.state('aod:logAssetsNotLoaded');
if((assetsLoadedPageState == null || !assetsLoadedPageState.isAodAssetsLoaded)
&amp;&amp; (logAssetsNotLoadedState == null || !logAssetsNotLoadedState.isAodAssetsNotLoadedLogged)) {
A.state('aod:logAssetsNotLoaded', {isAodAssetsNotLoadedLogged: true});
logAssetsNotLoaded();
}
}
setTimeout(verifyAssetsLoaded, 9000)
});
































































Have one to sell?






Sell on Amazon






















































































































































































































</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;rightCol&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='rightCol']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='ppd']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Back to results'])[1]/following::div[8]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='‹'])[1]/following::div[8]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[5]/div[5]/div</value>
   </webElementXpaths>
</WebElementEntity>
