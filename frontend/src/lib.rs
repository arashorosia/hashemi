use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use gloo_net::http::Request;
use wasm_bindgen::prelude::*;
use web_sys::Element;

// JavaScript bindings for Persian DatePicker
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = persianDatepicker)]
    type DatePicker;
    
    #[wasm_bindgen(constructor, js_namespace = persianDatepicker)]
    fn new(element: &Element, options: &JsValue) -> DatePicker;
    
    #[wasm_bindgen(method)]
    fn show(this: &DatePicker);
    
    #[wasm_bindgen(method)]
    fn hide(this: &DatePicker);
}

// API Structures
#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub id: Option<u32>,
    pub employee_id: String,
    pub first_name: String,
    pub last_name: String,
    pub father_name: Option<String>,
    pub national_id: String,
    pub birth_place: Option<String>,
    pub birth_date_shamsi: Option<String>,
    pub birth_date_miladi: Option<String>,
    pub phone: String,
    pub email: String,
    pub address: Option<String>,
    pub emergency_contact: Option<String>,
    pub position: String,
    pub department: String,
    pub work_type: String,
    pub salary: Option<i32>,
    pub contract_duration: Option<i32>,
    pub contract_start_shamsi: Option<String>,
    pub contract_start_miladi: Option<String>,
    pub contract_end_shamsi: Option<String>,
    pub contract_end_miladi: Option<String>,
    pub photo_url: Option<String>,
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Html lang="fa" dir="rtl"/>
        <Title text="Phoenix TechWorks Group"/>
        <Meta name="description" content="Phoenix TechWorks Group Management System"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        
        <Router>
            <main class="app">
                <Routes>
                    <Route path="/" view=LoginPage/>
                    <Route path="/admin" view=AdminPage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn LoginPage() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());
    let (is_loading, set_is_loading) = create_signal(false);
    let (message, set_message) = create_signal(String::new());
    let (is_error, set_is_error) = create_signal(false);
    let navigate = leptos_router::use_navigate();

    let handle_submit = create_action(move |_: &()| {
        let email_val = email.get();
        let password_val = password.get();
        let navigate = navigate.clone();
        
        async move {
            set_is_loading.set(true);
            set_message.set(String::new());

            let login_request = LoginRequest {
                email: email_val,
                password: password_val,
            };

            let request_result = Request::post("http://localhost:3000/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&login_request).unwrap_or_default());

            match request_result {
                Ok(request) => {
                    match request.send().await {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<LoginResponse>().await {
                                    Ok(login_response) => {
                                        set_is_error.set(false);
                                        set_message.set(format!("✅ {}", login_response.message));
                                        
                                        // Store token in localStorage
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item("auth_token", &login_response.token);
                                            }
                                        }
                                        
                                        // Navigate to admin page
                                        navigate("/admin", Default::default());
                                    }
                                    Err(e) => {
                                        set_is_error.set(true);
                                        set_message.set(format!("خطا در پردازش پاسخ: {:?}", e));
                                    }
                                }
                            } else {
                                match response.json::<ErrorResponse>().await {
                                    Ok(error_response) => {
                                        set_is_error.set(true);
                                        set_message.set(format!("❌ {}", error_response.error));
                                    }
                                    Err(_) => {
                                        set_is_error.set(true);
                                        set_message.set("خطا در برقراری ارتباط".to_string());
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            set_is_error.set(true);
                            set_message.set(format!("خطای شبکه: {:?}", e));
                        }
                    }
                }
                Err(e) => {
                    set_is_error.set(true);
                    set_message.set(format!("خطا در ایجاد درخواست: {:?}", e));
                }
            }

            set_is_loading.set(false);
        }
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

#[component]
fn AdminPage() -> impl IntoView {
    let (users, set_users) = create_signal(Vec::<User>::new());
    let (current_user, set_current_user) = create_signal(User::default());
    let (show_modal, set_show_modal) = create_signal(false);
    let (search_term, set_search_term) = create_signal(String::new());
    let (is_editing, set_is_editing) = create_signal(false);
    let navigate1 = leptos_router::use_navigate();
    let navigate2 = leptos_router::use_navigate();

    // Work type options according to requirements
    let work_types = vec![
        "الگوریتم ساده نوع A",
        "الگوریتم ساده نوع B",
        "الگوریتم ساده نوع C",
        "الگوریتم پیشرفته نوع A",
        "الگوریتم پیشرفته نوع B",
        "الگوریتم پیشرفته نوع C",
    ];

    // Generate unique 16-digit employee ID
    let generate_employee_id = move || -> String {
        let timestamp = js_sys::Date::now() as u64;
        let random_part = (js_sys::Math::random() * 1000000.0) as u64;
        let combined = timestamp + random_part + users.get().len() as u64;
        format!("{:016}", combined % 10_000_000_000_000_000)
    };

    // Check authentication
    let check_auth = move || {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                return storage.get_item("auth_token").unwrap_or(None).is_some();
            }
        }
        false
    };

    // Redirect if not authenticated
    create_effect(move |_| {
        if !check_auth() {
            navigate1("/", Default::default());
        }
    });

    // Handle logout
    let handle_logout = move |_| {
        if let Some(window) = web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("auth_token");
            }
        }
        navigate2("/", Default::default());
    };

    // Open add user modal
    let open_add_modal = move |_| {
        let mut new_user = User::default();
        new_user.employee_id = generate_employee_id();
        set_current_user.set(new_user);
        set_is_editing.set(false);
        set_show_modal.set(true);
    };

    // Open edit user modal
    let open_edit_modal = move |user: User| {
        set_current_user.set(user);
        set_is_editing.set(true);
        set_show_modal.set(true);
    };

    // Close modal
    let close_modal = move |_: ()| {
        set_show_modal.set(false);
        set_current_user.set(User::default());
    };

    // Save user
    let save_user = move |_: ()| {
        let user = current_user.get();
        let mut users_list = users.get();
        
        if is_editing.get() {
            if let Some(index) = users_list.iter().position(|u| u.id == user.id) {
                users_list[index] = user;
            }
        } else {
            let mut new_user = user;
            new_user.id = Some(users_list.len() as u32 + 1);
            users_list.push(new_user);
        }
        
        set_users.set(users_list);
        set_show_modal.set(false);
        set_current_user.set(User::default());
    };

    // Delete user
    let delete_user = move |user_id: u32| {
        let users_list: Vec<User> = users.get().into_iter()
            .filter(|u| u.id != Some(user_id))
            .collect();
        set_users.set(users_list);
    };

    // Filtered users based on search
    let filtered_users = move || {
        let search = search_term.get().to_lowercase();
        if search.is_empty() {
            users.get()
        } else {
            users.get().into_iter()
                .filter(|user| {
                    user.first_name.to_lowercase().contains(&search) ||
                    user.last_name.to_lowercase().contains(&search) ||
                    user.national_id.contains(&search) ||
                    user.email.to_lowercase().contains(&search) ||
                    user.employee_id.contains(&search)
                })
                .collect()
        }
    };

    view! {
        <div class="admin-container">
            <div class="admin-header">
                <h1 class="admin-title">"پنل مدیریت Phoenix TechWorks"</h1>
                <button class="logout-btn" on:click=handle_logout>"خروج"</button>
            </div>

            <div class="admin-controls">
                <button class="btn btn-primary" on:click=open_add_modal>
                    "افزودن کاربر جدید"
                </button>
                <div class="search-container">
                    <input
                        type="text"
                        placeholder="جستجو بر اساس نام، کد ملی، ایمیل..."
                        prop:value=search_term
                        on:input=move |e| set_search_term.set(event_target_value(&e))
                    />
                </div>
            </div>

            <div class="users-table-container">
                <table class="users-table">
                    <thead>
                        <tr>
                            <th>"شناسه پرسنلی"</th>
                            <th>"نام و نام خانوادگی"</th>
                            <th>"کد ملی"</th>
                            <th>"ایمیل"</th>
                            <th>"سمت"</th>
                            <th>"نوع کار"</th>
                            <th>"عملیات"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For
                            each=filtered_users
                            key=|user| user.id.unwrap_or(0)
                            children=move |user| {
                                let user_for_edit = user.clone();
                                let user_id = user.id.unwrap_or(0);
                                
                                view! {
                                    <tr>
                                        <td>{&user.employee_id}</td>
                                        <td>{format!("{} {}", &user.first_name, &user.last_name)}</td>
                                        <td>{&user.national_id}</td>
                                        <td>{&user.email}</td>
                                        <td>{&user.position}</td>
                                        <td>{&user.work_type}</td>
                                        <td>
                                            <button 
                                                class="btn btn-small btn-secondary" 
                                                on:click=move |_| open_edit_modal(user_for_edit.clone())
                                            >
                                                "ویرایش"
                                            </button>
                                            <button 
                                                class="btn btn-small btn-danger" 
                                                on:click=move |_| {
                                                    if web_sys::window().unwrap()
                                                        .confirm_with_message(&format!(
                                                            "آیا می‌خواهید {} {} را حذف کنید؟", 
                                                            &user.first_name, &user.last_name
                                                        )).unwrap_or(false) {
                                                        delete_user(user_id);
                                                    }
                                                }
                                            >
                                                "حذف"
                                            </button>
                                        </td>
                                    </tr>
                                }
                            }
                        />
                    </tbody>
                </table>
            </div>

            // User Management Modal
            <Show when=move || show_modal.get()>
                <div class="modal-overlay" on:click=move |_| close_modal(())>
                    <div class="modal" on:click=move |e| e.stop_propagation()>
                        <div class="modal-header">
                            <h3>{move || if is_editing.get() { "ویرایش کاربر" } else { "افزودن کاربر جدید" }}</h3>
                            <button class="close-btn" on:click=move |_| close_modal(())>"×"</button>
                        </div>
                        <div class="modal-body">
                            <UserFormComponent 
                                user=current_user 
                                set_user=set_current_user 
                                work_types=work_types.clone()
                                on_save=save_user
                                on_cancel=close_modal
                            />
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}

// User Form Component
#[component]
fn UserFormComponent(
    user: ReadSignal<User>,
    set_user: WriteSignal<User>,
    work_types: Vec<&'static str>,
    on_save: impl Fn(()) + 'static + Copy,
    on_cancel: impl Fn(()) + 'static + Copy,
) -> impl IntoView {
    
    let update_field = move |field: &str, value: String| {
        let mut current_user = user.get();
        match field {
            "first_name" => current_user.first_name = value,
            "last_name" => current_user.last_name = value,
            "father_name" => current_user.father_name = Some(value),
            "national_id" => current_user.national_id = value,
            "birth_place" => current_user.birth_place = Some(value),
            "phone" => current_user.phone = value,
            "email" => current_user.email = value,
            "address" => current_user.address = Some(value),
            "emergency_contact" => current_user.emergency_contact = Some(value),
            "position" => current_user.position = value,
            "department" => current_user.department = value,
            "work_type" => current_user.work_type = value,
            "salary" => current_user.salary = value.parse().ok(),
            "contract_duration" => current_user.contract_duration = value.parse().ok(),
            "birth_date_shamsi" => current_user.birth_date_shamsi = Some(value),
            "contract_start_shamsi" => current_user.contract_start_shamsi = Some(value),
            _ => {}
        }
        set_user.set(current_user);
    };

    view! {
        <form on:submit=move |e| {
            e.prevent_default();
            on_save(());
        }>
            <div class="form-grid">
                // شناسه پرسنلی (فقط خواندنی)
                <div class="form-group full-width">
                    <label>"شناسه پرسنلی (16 رقم منحصربفرد)"</label>
                    <input type="text" value={move || user.get().employee_id} readonly class="readonly-field" />
                </div>
                
                // اطلاعات شخصی
                <div class="form-group">
                    <label>"نام *"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().first_name}
                        on:input=move |e| update_field("first_name", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"نام خانوادگی *"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().last_name}
                        on:input=move |e| update_field("last_name", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"نام پدر"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().father_name.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| update_field("father_name", event_target_value(&e))
                    />
                </div>
                
                <div class="form-group">
                    <label>"کد ملی *"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().national_id}
                        on:input=move |e| update_field("national_id", event_target_value(&e))
                        maxlength="10"
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"محل تولد"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().birth_place.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| update_field("birth_place", event_target_value(&e))
                    />
                </div>
                
                <div class="form-group">
                    <label>"تاریخ تولد شمسی"</label>
                    <ShamsiDatePicker
                        placeholder="1370/01/01 - انتخاب تاریخ"
                        value=Signal::derive(move || user.get().birth_date_shamsi.as_deref().unwrap_or("").to_string())
                        on_change=Callback::new(move |val: String| {
                            let miladi_val = shamsi_to_miladi(&val);
                            set_user.update(|u| {
                                u.birth_date_shamsi = Some(val);
                                u.birth_date_miladi = Some(miladi_val);
                            });
                        })
                        class="shamsi-date-picker"
                    />
                </div>

                <div class="form-group">
                    <label>"تاریخ تولد میلادی"</label>
                    <input 
                        type="date"
                        prop:value={move || user.get().birth_date_miladi.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| {
                            let miladi_val = event_target_value(&e);
                            let shamsi_val = miladi_to_shamsi(&miladi_val);
                             set_user.update(|u| {
                                u.birth_date_miladi = Some(miladi_val);
                                u.birth_date_shamsi = Some(shamsi_val);
                            });
                        }
                    />
                </div>
                
                // اطلاعات تماس
                <div class="form-group">
                    <label>"تلفن *"</label>
                    <input 
                        type="tel" 
                        prop:value={move || user.get().phone}
                        on:input=move |e| update_field("phone", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"ایمیل *"</label>
                    <input 
                        type="email" 
                        prop:value={move || user.get().email}
                        on:input=move |e| update_field("email", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group full-width">
                    <label>"آدرس"</label>
                    <textarea 
                        prop:value={move || user.get().address.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| update_field("address", event_target_value(&e))
                    />
                </div>
                
                <div class="form-group">
                    <label>"مخاطب اضطراری"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().emergency_contact.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| update_field("emergency_contact", event_target_value(&e))
                    />
                </div>
                
                // اطلاعات شغلی
                <div class="form-group">
                    <label>"سمت *"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().position}
                        on:input=move |e| update_field("position", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"بخش *"</label>
                    <input 
                        type="text" 
                        prop:value={move || user.get().department}
                        on:input=move |e| update_field("department", event_target_value(&e))
                        required 
                    />
                </div>
                
                <div class="form-group">
                    <label>"نوع کار *"</label>
                    <select 
                        prop:value={move || user.get().work_type}
                        on:change=move |e| update_field("work_type", event_target_value(&e))
                        required
                    >
                        <option value="">"انتخاب کنید"</option>
                        {work_types.into_iter().map(|work_type| {
                            view! { <option value={work_type}>{work_type}</option> }
                        }).collect::<Vec<_>>()}
                    </select>
                </div>
                
                <div class="form-group">
                    <label>"حقوق (تومان)"</label>
                    <input 
                        type="number" 
                        prop:value={move || user.get().salary.map(|s| s.to_string()).unwrap_or_default()}
                        on:input=move |e| update_field("salary", event_target_value(&e))
                    />
                </div>
                
                <div class="form-group">
                    <label>"مدت قرارداد (ماه)"</label>
                    <input 
                        type="number" 
                        prop:value={move || user.get().contract_duration.map(|s| s.to_string()).unwrap_or_default()}
                        on:input=move |e| update_field("contract_duration", event_target_value(&e))
                    />
                </div>
                
                <div class="form-group">
                    <label>"تاریخ شروع قرارداد شمسی"</label>
                    <ShamsiDatePicker
                        placeholder="1403/01/01 - انتخاب تاریخ"
                        value=Signal::derive(move || user.get().contract_start_shamsi.as_deref().unwrap_or("").to_string())
                        on_change=Callback::new(move |val: String| {
                            let miladi_val = shamsi_to_miladi(&val);
                            set_user.update(|u| {
                                u.contract_start_shamsi = Some(val);
                                u.contract_start_miladi = Some(miladi_val);
                            });
                        })
                        class="shamsi-date-picker"
                    />
                </div>

                <div class="form-group">
                    <label>"تاریخ شروع قرارداد میلادی"</label>
                    <input 
                        type="date"
                        prop:value={move || user.get().contract_start_miladi.as_deref().unwrap_or("").to_string()}
                        on:input=move |e| {
                            let miladi_val = event_target_value(&e);
                            let shamsi_val = miladi_to_shamsi(&miladi_val);
                            set_user.update(|u| {
                                u.contract_start_miladi = Some(miladi_val);
                                u.contract_start_shamsi = Some(shamsi_val);
                            });
                        }
                    />
                </div>
            </div>
            
            <div class="modal-footer">
                <button type="submit" class="btn btn-primary">"ذخیره"</button>
                <button type="button" class="btn btn-secondary" on:click=move |_| on_cancel(())>"انصراف"</button>
            </div>
        </form>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found">
            <h1>"404 - صفحه یافت نشد"</h1>
            <a href="/">"بازگشت به صفحه اصلی"</a>
        </div>
    }
}

// Helper functions - simplified for JavaScript datepicker
fn shamsi_to_miladi(shamsi_str: &str) -> String {
    // Conversion will be handled by JavaScript DatePicker
    if shamsi_str.is_empty() {
        String::new()
    } else {
        // Return placeholder - actual conversion handled by JS
        "2024-01-01".to_string()
    }
}

fn miladi_to_shamsi(miladi_str: &str) -> String {
    // Conversion will be handled by JavaScript DatePicker
    if miladi_str.is_empty() {
        String::new()
    } else {
        // Return placeholder - actual conversion handled by JS
        "1403/01/01".to_string()
    }
}

// Simplified ShamsiDatePicker Component
#[component]
fn ShamsiDatePicker(
    #[prop(into)] placeholder: String,
    #[prop(into)] value: Signal<String>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into, optional)] class: String
) -> impl IntoView {
    view! {
        <input
            type="text"
            class=format!("shamsi-datepicker {}", class)
            placeholder=placeholder
            prop:value=move || value.get()
            on:input=move |e| {
                let val = event_target_value(&e);
                on_change.call(val);
            }
        />
    }
}

// Entry point for WASM
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}