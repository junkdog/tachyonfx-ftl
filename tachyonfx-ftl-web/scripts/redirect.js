// check if we need to redirect; called after trunk is started
function encodeDefaultCodeAndCanvasOnMissing() {
    console.log("checkAndRedirect called");
    const params = new URLSearchParams(window.location.search);
    if (!params.has("code") && !params.has("canvas")) {
        const url = new URL(window.location);
        url.searchParams.set("code", 'JYtBDsIgEEX3nIIlJKYBGjcYV17ApAcgtR1aklEamNp6e0f7Nz__5z0EkoO8ylvGXLyPJT_D2jpldjs646y-CGSk0geBse7X3o8Q-xVJ6eYxqYEZEXd-U60Z3xAoq79wktYYo4XkNFuiOSw9EZSX6jaA5X4M7xEisRRKmmZS7Vlr8QU');
        url.searchParams.set("canvas", '7VdRjtowEP3vFfiZAyBECHSh-WvVVSu1UtWt1Ep8DcaQkZw4azss_PUsPVpP0rEDWrqs2IBYdpFiCQj2OLx588aPtMbdrDXuD5Ne0hslcS-J4wzONVr8429aDYIGQYOgFoKt9UE34c_-1ckRPsnC8UiryThsGvSTqMdXg52wHyjSlc6vf22HR_1kFCdRf_emH2czKZyFm1TfCbTyZBw9IxEnHi9estY4ylxVttlya0s0HCXRFW8LMQ9uTRYwB7kuH-ZTflGGjnQOiiYGzQpm2sB3dOhKAiwKRSKs284-Ws9Dx77c3pekpiB0Vii5vE_LwmQVZrWlfB5SVriSxn-x5IM3dHRqImnUWascnuFAcqa1S8EZzC1VFfFFWJAtUUGhFVle1eBSCU6ajHJUnT35nKUcNZP8iU6kYAtWms8FHSfA-QmdO6PVWljgKNtIb0rWGZqUnocXT_LEo0H68OzuBvE8oht_dv_9_ee_6HjAjuulNnw0_KsOh_S3tca2Bfp2mEQDNuq4t7PzHdg7KYs2WEVT2Qa5LFiGba9Dq9XiCOO-RO4-SKXgmhQzdwBxUnHz0oJPJTRz6XwLBytR2rTB24kvyEEEXiR3lXXe51qDugINKiUVy07eljIXrDwjC4mBxLXdHii9S-Tuc86KY4er_ojUpu-2RG5RUU5IhI7V_K7QOhI8aRbyUOouVHfcZ5sHjdrUfbr5AjalmbNtmBucksz9pVgJxdI7xlsaV3utSJ91NAgaBA2CV4XgHw');
        window.location.href = url.toString();
    }
}