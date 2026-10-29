import React, { useState } from 'react';
import Link from 'next/link';
import Header from '../components/Header';
import SearchModal from '../components/SearchModal';
import CodeBlock from '../components/CodeBlock';
import { 
  Download, Terminal, Heart, Sparkles, Bot, CheckCircle, 
  ArrowLeft, ShieldCheck, Box, Package, Cpu, Code2
} from 'lucide-react';

export default function PersianDocsPage() {
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  return (
    <div className="min-h-screen bg-slate-950 text-slate-100 flex flex-col font-vazir selection:bg-emerald-500/30 selection:text-emerald-200" dir="rtl">
      <Header onOpenSearch={() => setIsSearchOpen(true)} />
      <SearchModal isOpen={isSearchOpen} onClose={() => setIsSearchOpen(false)} />

      <main className="mx-auto max-w-4xl px-4 sm:px-6 lg:px-8 py-10 flex-1">
        
        {/* AI Disclaimer */}
        <div className="mb-6 flex items-center justify-between gap-3 rounded-xl border border-purple-500/30 bg-purple-500/10 px-4 py-2.5 text-xs text-purple-300">
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4 text-purple-400 shrink-0" />
            <span>این وب‌سایت با کمک هوش مصنوعی ساخته شده است • Built with AI</span>
          </div>
          <Link href="/docs/installation" className="text-purple-300 hover:text-white underline font-sans text-[11px]">
            English Docs
          </Link>
        </div>

        {/* Hero Header */}
        <header className="border-b border-slate-800 pb-8 mb-10 text-right">
          <div className="inline-block rounded-full border border-emerald-500/30 bg-emerald-500/10 px-3 py-1 text-xs font-semibold text-emerald-400 mb-4">
            نسخه 0.13.0 پایدار • انتشار رسمی
          </div>
          <h1 className="text-3xl sm:text-5xl font-extrabold text-white tracking-tight">
            راهنمای جامع زبان برنامه‌نویسی پرسیکا (Persica) 🌿
          </h1>
          <p className="mt-4 text-base text-slate-300 leading-relaxed max-w-3xl">
            پرسیکا یک زبان برنامه‌نویسی مدرن، ایستا-نوع (Statically-Typed) و کاملاً مستقل است که از صفر تا صد با زبان Rust پیاده‌سازی شده است. این زبان زیبایی و سادگی سینتکس <strong>پایتون</strong> را با ایمنی و قابلیت اطمینان سیستم‌های تایپ‌شده ترکیب کرده است.
          </p>
        </header>

        {/* Quick Links Card */}
        <div className="my-8 rounded-2xl border border-slate-800 bg-slate-900/60 p-6 text-sm text-slate-300">
          <h3 className="font-bold text-emerald-400 text-base mb-3">فهرست موضوعات راهنما:</h3>
          <ul className="grid grid-cols-1 sm:grid-cols-2 gap-2 text-xs">
            <li><a href="#install" className="hover:text-emerald-300">۱. نصب نصاب ویندوز و کامپایل سورس</a></li>
            <li><a href="#quickstart" className="hover:text-emerald-300">۲. اجرای بدون تشریفات فایل تکی</a></li>
            <li><a href="#project-system" className="hover:text-emerald-300">۳. سیستم پروژه persica.toml و محیط .persi_env</a></li>
            <li><a href="#types" className="hover:text-emerald-300">۴. انواع داده‌ها، متغیرها و final</a></li>
            <li><a href="#oop" className="hover:text-emerald-300">۵. مدل شیء‌گرایی: struct و class</a></li>
            <li><a href="#exceptions" className="hover:text-emerald-300">۶. مدیریت خطا و استثناها (try / catch / throw)</a></li>
            <li><a href="#stdlib" className="hover:text-emerald-300">۷. کتابخانه استاندارد خودمیزبان و Prelude</a></li>
            <li><a href="#vscode" className="hover:text-emerald-300">۸. افزونه رسمی وی‌اس‌کد (VS Code Extension)</a></li>
            <li><a href="#sponsor" className="hover:text-emerald-300">۹. حمایت مالی از پروژه (رِیمیت)</a></li>
          </ul>
        </div>

        {/* Section 1: Installation */}
        <section id="install" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۱. نصب و راه‌اندازی (Windows)
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-4">
            ساده‌ترین راهکار، دانلود فایل نصاب خودکار رسمی است که مسیر کامپایلر را خودکار در PATH ویندوز اضافه می‌کند و پسوند فایل‌های <code className="text-emerald-300">.persi</code> را در سیستم رجیستر می‌نماید:
          </p>
          <div className="my-4">
            <a
              href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/Persica-0.13.0-Windows-x64-Setup.exe"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-emerald-500 hover:bg-emerald-600 text-white font-bold text-xs shadow-lg shadow-emerald-500/20 transition"
            >
              <Download className="w-4 h-4" />
              <span>دانلود نصاب رسمی ویندوز (Persica-0.13.0-Windows-x64-Setup.exe)</span>
            </a>
          </div>

          <p className="text-xs text-slate-400 mt-4 mb-2">تست نسخه در ترمینال:</p>
          <CodeBlock
            language="bash"
            filename="Terminal"
            code={`persica --version`}
            output={`persica 0.13.0`}
          />
        </section>

        {/* Section 2: Quickstart */}
        <section id="quickstart" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۲. اجرای بدون تشریفات فایل تکی (Zero-Ceremony)
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            برای نوشتن اسکریپت‌های سبک یا تست قطعه کد، نیازی به تعریف کلاس یا ساختار پروژه ندارید. یک فایل به نام <code className="text-emerald-400">test.persi</code> بسازید:
          </p>
          <CodeBlock
            language="persica"
            filename="test.persi"
            code={`// چاپ پیام و اجرای یک حلقه ساده
print("درود بر برنامه‌نویسان پرسیکا! 🌿");

int sum = 0;
for i in range(1, 6) {
    sum += i;
    print("گام:", i, "مجموع فعلی:", sum);
}`}
            output={`درود بر برنامه‌نویسان پرسیکا! 🌿
گام: 1 مجموع فعلی: 1
گام: 2 مجموع فعلی: 3
گام: 3 مجموع فعلی: 6
گام: 4 مجموع فعلی: 10
گام: 5 مجموع فعلی: 15`}
          />
        </section>

        {/* Section 3: Project System */}
        <section id="project-system" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۳. سیستم پروژه persica.toml و محیط ایزوله .persi_env
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            برای پروژه‌های چندفایله بزرگ، دستور <code className="text-emerald-400">persica new</code> ساختار کامل پروژه را می‌سازد:
          </p>
          <CodeBlock
            language="bash"
            filename="Terminal"
            code={`persica new my_app
cd my_app
persica run`}
          />

          <p className="text-xs text-slate-300 mt-4 mb-2 font-semibold">محتوای فایل persica.toml:</p>
          <CodeBlock
            language="toml"
            filename="persica.toml"
            code={`[package]
name = "my_app"
version = "0.1.0"
entry = "src/main.persi"
source = "src"

[environment]
env_dir = ".persi_env"`}
          />
          <p className="text-xs text-slate-400 mt-2">
            محیط <code className="text-emerald-300">.persi_env</code> پکیج‌های شما را به صورت ایزوله نگهداری می‌کند بدون آن که فایل‌های اجرایی سنگین و تکراری مفسر کپی شوند.
          </p>
        </section>

        {/* Section 4: Types & Variables */}
        <section id="types" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۴. انواع داده‌ها و متغیرها
          </h2>
          <CodeBlock
            language="persica"
            filename="types.persi"
            code={`int age = 22;
float pi = 3.14159;
string lang = "Persica";
bool is_fast = true;

// متغیرهای غیرقابل تغییر با کلمه کلیدی final
final string VERSION = "0.13.0";

// تبدیل صریح انواع
int rounded = to_int(pi);
string text = to_string(age) + " years";`}
          />
        </section>

        {/* Section 5: OOP */}
        <section id="oop" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۵. شیء‌گرایی: struct و class
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            پرسیکا تمایز دقیقی میان ساختار داده‌ای مقداری خالص (<code className="text-blue-300">struct</code>) و اشیاء ارجاعی دارای متد و رفتار (<code className="text-emerald-300">class</code>) قائل است:
          </p>
          <CodeBlock
            language="persica"
            filename="oop.persi"
            code={`// مدل داده‌ای خالص (Value Semantics)
struct Point {
    int x;
    int y;
}

// شیء دارای متد و رفتار (Reference Semantics)
class Wallet {
    string owner;
    int balance = 0;

    void add_funds(int amount) {
        this.balance += amount;
    }

    int get_balance() {
        return this.balance;
    }
}

Wallet w = Wallet();
w.owner = "محمد طاها";
w.add_funds(2500);
print("موجودی کیف پول:", w.get_balance());`}
            output={`موجودی کیف پول: 2500`}
          />
        </section>

        {/* Section 6: Exceptions */}
        <section id="exceptions" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۶. مهار خطا و استثناها (try / catch / throw)
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            تمامی خطاهای درونی رانتایم (تقسیم بر صفر، اندیس خارج از محدوده آرایه) و خطاهای سفارشی پرتاب‌شده توسط برنامه، بدون کرش کردن مفسر در بلوک <code className="text-emerald-400">catch</code> مهار می‌شوند:
          </p>
          <CodeBlock
            language="persica"
            filename="exceptions.persi"
            code={`try {
    int denom = 0;
    if denom == 0 {
        throw "خطا: تقسیم بر صفر مجاز نیست!";
    }
} catch (error_msg) {
    print("خطا مهار شد:", error_msg);
}`}
            output={`خطا مهار شد: خطا: تقسیم بر صفر مجاز نیست!`}
          />
        </section>

        {/* Section 7: Stdlib */}
        <section id="stdlib" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۷. کتابخانه استاندارد خودمیزبان و Prelude
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            ساختارهای پشته (Stack)، صف (Queue)، مجموعه (Set) و بافر رشته (StringBuilder) همیشه به صورت پیش‌درآمد در تمام برنامه‌ها بدون نیاز به ایمپورت در دسترس هستند:
          </p>
          <CodeBlock
            language="persica"
            filename="prelude.persi"
            code={`Stack stack = Stack();
stack.push("آیتم اول");
stack.push("آیتم دوم");
print("خارج شده از پشته:", stack.pop());

from std.math import sqrt;
print("جذر عدد 64 برابر است با:", sqrt(64.0));`}
            output={`خارج شده از پشته: آیتم دوم
جذر عدد 64 برابر است با: 8`}
          />
        </section>

        {/* Section 8: VS Code */}
        <section id="vscode" className="my-12">
          <h2 className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mb-4">
            ۸. افزونه رسمی وی‌اس‌کد (VS Code Extension)
          </h2>
          <p className="text-sm text-slate-300 leading-relaxed mb-3">
            افزونه رسمی زبان پرسیکا شامل قابلیت‌های تکمیل خودکار هوشمند، هایلایت دقیق سینتکس، مستندات هاور و چک خطای زنده با ذخیره فایل است. برای نصب فایل <code className="text-emerald-300">persica-0.13.0.vsix</code> را دریافت و نصب کنید:
          </p>
          <div className="my-3">
            <a
              href="https://github.com/taha-heidaryfard/Persica/releases/download/v0.13.0/persica-0.13.0.vsix"
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl border border-slate-700 bg-slate-900 hover:bg-slate-800 text-xs font-bold text-white transition"
            >
              <Download className="w-4 h-4 text-emerald-400" />
              <span>دانلود افزونه persica-0.13.0.vsix</span>
            </a>
          </div>
        </section>

        {/* Section 9: Reymit Sponsor */}
        <section id="sponsor" className="my-14">
          <div className="rounded-3xl border border-rose-500/30 bg-gradient-to-r from-rose-950/20 via-pink-950/20 to-slate-900/60 p-8 text-center shadow-xl">
            <Heart className="w-8 h-8 text-rose-500 fill-rose-500 mx-auto mb-3 animate-pulse" />
            <h2 className="text-xl sm:text-2xl font-bold text-white">
              حمایت مالی از توسعه زبان پرسیکا
            </h2>
            <p className="mt-3 text-xs sm:text-sm text-slate-300 max-w-xl mx-auto leading-relaxed">
              زبان پرسیکا توسط <strong className="text-white">محمد طاها حیدری‌فرد (Taxtander)</strong> به صورت کاملاً مستقل توسعه می‌یابد. اگر این پروژه برای شما جالب یا کاربردی بوده، از طریق درگاه رِیمیت می‌توانید از توسعه آن حمایت کنید:
            </p>
            <div className="mt-5">
              <a
                href="https://reymit.ir/taxtander"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 rounded-xl bg-gradient-to-r from-rose-500 to-pink-600 px-6 py-3 text-xs font-bold text-white shadow-lg shadow-rose-500/25 hover:from-rose-600 hover:to-pink-700 transition"
              >
                <span>ورود به درگاه رِیمیت (Reymit)</span>
                <ArrowLeft className="w-4 h-4" />
              </a>
            </div>
          </div>
        </section>

        {/* Footer */}
        <footer className="border-t border-slate-900 pt-8 pb-16 text-center text-xs text-slate-400 space-y-2">
          <p>طراحی و توسعه داده شده توسط <strong>محمد طاها حیدری‌فرد (Taxtander)</strong> • تحت مجوز MIT</p>
          <p className="text-[11px] text-slate-400">🤖 این وب‌سایت با کمک هوش مصنوعی ساخته شده است • Built with AI</p>
          <div className="pt-2">
            <Link href="/" className="text-emerald-400 hover:underline">بازگشت به صفحه اصلی</Link>
          </div>
        </footer>

      </main>
    </div>
  );
}
