use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

// API Request/Response Structures
#[derive(Serialize, Deserialize, Clone)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct LoginResponse {
    token: String,
    message: String,
}

#[derive(Serialize, Deserialize, Clone)]  
struct ErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct User {
    id: Option<u32>,
    employee_id: String,
    first_name: String,
    last_name: String,
    father_name: Option<String>,
    national_id: String,
    birth_place: Option<String>,
    birth_date_shamsi: Option<String>,
    birth_date_miladi: Option<String>,
    phone: String,
    email: String,
    address: Option<String>,
    emergency_contact: Option<String>,
    position: String,
    department: String,
    work_type: String,
    salary: Option<i32>,
    contract_duration: Option<i32>,
    contract_start_shamsi: Option<String>,
    contract_start_miladi: Option<String>,
    contract_end_shamsi: Option<String>,
    contract_end_miladi: Option<String>,
    photo_url: Option<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: None,
            employee_id: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            father_name: None,
            national_id: String::new(),
            birth_place: None,
            birth_date_shamsi: None,
            birth_date_miladi: None,
            phone: String::new(),
            email: String::new(),
            address: None,
            emergency_contact: None,
            position: String::new(),
            department: String::new(),
            work_type: String::new(),
            salary: None,
            contract_duration: None,
            contract_start_shamsi: None,
            contract_start_miladi: None,
            contract_end_shamsi: None,
            contract_end_miladi: None,
            photo_url: None,
        }
    }
}

// Login Page Component
#[component]
pub fn login_page() -> impl IntoView {
    let (email, set_email) = create_signal("admin@phoenixtechworks.com".to_string());
    let (password, set_password) = create_signal("Phoenix2025!".to_string());
    let (message, set_message) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);
    let (is_error, set_is_error) = create_signal(false);

    let handle_submit = create_action(move |_: &()| async move {
        set_is_loading.set(true);
        set_message.set(String::new());

        let login_data = LoginRequest {
            email: email.get(),
            password: password.get(),
        };

        let request_result = Request::post("http://localhost:3000/login")
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&login_data).unwrap_or_default());
            
        match request_result {
            Ok(request) => {
                match request.send().await {
                    Ok(response) => {
                        if response.ok() {
                            match response.json::<LoginResponse>().await {
                                Ok(data) => {
                                    set_is_error.set(false);
                                    set_message.set(format!("✅ {}", data.message));
                                    
                                    // Store token and redirect
                                    if let Some(window) = web_sys::window() {
                                        if let Ok(Some(storage)) = window.local_storage() {
                                            let _ = storage.set_item("auth_token", &data.token);
                                        }
                                        // Redirect to admin page
                                        let _ = window.location().set_href("/admin");
                                    }
                                }
                                Err(e) => {
                                    set_is_error.set(true);
                                    set_message.set(format!("Parse error: {:?}", e));
                                }
                            }
                        } else {
                            match response.json::<ErrorResponse>().await {
                                Ok(err_data) => {
                                    set_is_error.set(true);
                                    set_message.set(format!("❌ {}", err_data.error));
                                }
                                Err(_) => {
                                    set_is_error.set(true);
                                    set_message.set("Request failed".to_string());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        set_is_error.set(true);
                        set_message.set(format!("Network error: {:?}", e));
                    }
                }
            }
            Err(e) => {
                set_is_error.set(true);
                set_message.set(format!("Request creation error: {:?}", e));
            }
        }
        
        set_is_loading.set(false);
    });

    view! {
        <div class="login-container">
            <div class="logo">
                <div class="logo-icon">"🔐"</div>
                <h1>"Phoenix TechWorks Group"</h1>
                <p>"پورتال دسترسی امن"</p>
            </div>
            
            <div class="company-info">
                <h3>"راه‌حل‌های فناوری کارت"</h3>
                <p>"سیستم‌های کدگذاری/کدگشایی کارت‌های مغناطیسی و تماسی"</p>
            </div>
            
            <form on:submit=move |e| {
                e.prevent_default();
                handle_submit.dispatch(());
            }>
                <div class="form-group">
                    <label for="email">"آدرس ایمیل"</label>
                    <input
                        type="email"
                        id="email"
                        placeholder="ایمیل خود را وارد کنید"
                        prop:value=email
                        on:input=move |e| set_email.set(event_target_value(&e))
                        required
                    />
                </div>
                
                <div class="form-group">
                    <label for="password">"رمز عبور"</label>
                    <input
                        type="password"
                        id="password"
                        placeholder="رمز عبور خود را وارد کنید"
                        prop:value=password
                        on:input=move |e| set_password.set(event_target_value(&e))
                        required
                    />
                </div>
                
                <button type="submit" class="login-btn" disabled=is_loading>
                    {move || if is_loading.get() { "در حال احراز هویت..." } else { "ورود به سیستم" }}
                </button>
            </form>
            
            <Show when=move || !message.get().is_empty()>
                <div class=move || format!("message {}", if is_error.get() { "error" } else { "success" })>
                    {message}
                </div>
            </Show>
            
            <div class="footer-info">
                <div>"© 1403 Phoenix TechWorks Group. تمامی حقوق محفوظ است."</div>
                <div class="security-notice">"امنیت تضمین شده با رمزگذاری سطح سازمانی"</div>
            </div>
        </div>
    }
}
        {
            Ok(response) => {
                if response.ok() {
                    match response.json::<LoginResponse>().await {
                        Ok(data) => {
                            set_is_error.set(false);
                            set_message.set(format!("✅ {}", data.message));
                            set_is_authenticated.set(true);
                            // Store token in localStorage
                            let window = web_sys::window().unwrap();
                            let storage = window.local_storage().unwrap().unwrap();
                            let _ = storage.set_item("auth_token", &data.token);
                        }
                        Err(e) => {
                            set_is_error.set(true);
                            set_message.set(format!("Parse error: {:?}", e));
                        }
                    }
                } else {
                    match response.json::<ErrorResponse>().await {
                        Ok(err_data) => {
                            set_is_error.set(true);
                            set_message.set(format!("❌ {}", err_data.error));
                        }
                        Err(_) => {
                            set_is_error.set(true);
                            set_message.set("Request failed".to_string());
                        }
                    }
                }
            }
            Err(e) => {
                set_is_error.set(true);
                set_message.set(format!("Network error: {:?}", e));
            }
        }
        
        set_is_loading.set(false);
    });

    view! {
        <Html lang="fa" dir="ltr"/>
        <Title text="Phoenix TechWorks Group - Login Portal"/>
        <Meta name="description" content="Phoenix TechWorks Group - Secure login portal for card technology systems"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        
        <div class="app">
            <div class="login-container">
                <div class="logo">
                    <div class="logo-icon">"🔐"</div>
                    <h1>"Phoenix TechWorks Group"</h1>
                    <p>"Secure Access Portal"</p>
                </div>
                
                <div class="company-info">
                    <h3>"Card Technology Solutions"</h3>
                    <p>"Magnetic & Contactless Card Encoding/Decoding Systems"</p>
                </div>
                
                <form on:submit=move |e| {
                    e.prevent_default();
                    handle_submit.dispatch(());
                }>
                    <div class="form-group">
                        <label for="email">"Email Address"</label>
                        <input
                            type="email"
                            id="email"
                            placeholder="Enter your email"
                            prop:value=email
                            on:input=move |e| set_email.set(event_target_value(&e))
                            required
                        />
                    </div>
                    
                    <div class="form-group">
                        <label for="password">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            placeholder="Enter your password"
                            prop:value=password
                            on:input=move |e| set_password.set(event_target_value(&e))
                            required
                        />
                    </div>
                    
                    <button type="submit" class="login-btn" disabled=is_loading>
                        {move || if is_loading.get() { "Authenticating..." } else { "Access System" }}
                    </button>
                </form>
                
                <Show when=move || !message.get().is_empty()>
                    <div class=move || format!("message {}", if is_error.get() { "error" } else { "success" })>
                        {message}
                    </div>
                </Show>
                
                <div class="footer-info">
                    <div>"© 2025 Phoenix TechWorks Group. All rights reserved."</div>
                    <div class="security-notice">"Secured with enterprise-grade encryption"</div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet id="leptos" href="/pkg/frontend.css"/>
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=LoginPage/>
                </Routes>
            </main>
        </Router>
    }
}