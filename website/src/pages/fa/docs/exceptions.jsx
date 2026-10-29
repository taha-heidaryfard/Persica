import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { ShieldCheck, AlertTriangle } from 'lucide-react';

export default function ExceptionsPageFa() {
  const meta = {
    title: 'مدیریت خطا و استثناها (try / catch / throw)',
    category: 'شیء‌گرایی و پایداری',
    description: 'چگونگی مهار ایمن خطاها در پرسیکا، ساختارهای try و catch، پرتاب استثنا با throw و بازپیچی پشته فراخوانی (Stack Unwinding) با تضمین عدم پنیک.',
  };

  const toc = [
    { title: 'تضمین عدم پنیک (Zero-Panic Guarantee)', id: 'guarantee', level: 2 },
    { title: 'ساختار پایه try و catch', id: 'basic', level: 2 },
    { title: 'پرتاب خطا با throw', id: 'throw', level: 2 },
    { title: 'انواع سینتکس بلوک catch', id: 'variants', level: 2 },
    { title: 'مهار خودکار خطاهای رانتایم توکار', id: 'runtime-trapping', level: 2 },
    { title: 'بازپیچی پشته فراخوانی (Call-Stack Unwinding)', id: 'unwinding', level: 2 },
    { title: 'پرتاب مجدد خطا (Re-throwing)', id: 'rethrow', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="guarantee" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        تضمین عدم پنیک (Zero-Panic Guarantee)
      </h2>
      <p>
        در پرسیکا، <strong>هیچ خطایی در کد کاربر نباید منجر به کرش یا پنیک مفسر و رانتایم شود</strong>. مدل مدیریت استثنای زبان به شما اجازه می‌دهد هم خطاهای منطقی برنامه و هم خطاهای ذاتی نظیر تقسیم بر صفر یا خروج از محدوده آرایه را با امنیت کامل مهار نمایید.
      </p>

      <h2 id="basic" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ساختار پایه `try` و `catch`
      </h2>
      <p>
        کدهای حساس را درون بلوک <code className="text-emerald-400 font-bold">try</code> قرار داده و سپس مهار خطا را در بلوک <code className="text-emerald-400 font-bold">catch</code> مدیریت کنید:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="basic_try.persi"
          code={`try {
    print("Executing sensitive operation...");
    throw "Database unreachable!";
    print("This line will never be reached.");
} catch (error) {
    print("Intercepted error message:", error);
}

print("Application recovered smoothly! ✨");`}
          output={`Executing sensitive operation...
Intercepted error message: Database unreachable!
Application recovered smoothly! ✨`}
        />
      </div>

      <h2 id="throw" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        پرتاب خطا با `throw`
      </h2>
      <p>
        می‌توانید هر مقداری را به عنوان استثنا پرتاب کنید: رشته توضیحی، کد خطای عددی، دیکشنری ساختاریافته یا اشیاء سفارشی:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="throwing.persi"
          code={`// پرتاب کد خطای عددی
try {
    throw 404;
} catch (code) {
    print("Caught HTTP Error Code:", code);
}

// پرتاب دیکشنری حاوی جزئیات خطا
try {
    throw {
        "status": 500,
        "detail": "Failed to serialize JSON payload"
    };
} catch (err_dict) {
    print("Status:", err_dict["status"]);
    print("Detail:", err_dict["detail"]);
}`}
          output={`Caught HTTP Error Code: 404
Status: 500
Detail: Failed to serialize JSON payload`}
        />
      </div>

      <h2 id="variants" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        انواع سینتکس بلوک catch
      </h2>
      <p>
        تحلیل‌گر پرسیکا از چندین فرمت استاندارد و منعطف برای بلوک catch پشتیبانی می‌کند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="syntax_styles.persi"
          code={`// ۱. مدل استاندارد با پرانتز
try { throw "err1"; } catch (e) { print(e); }

// ۲. مدل پایتونی بدون پرانتز
try { throw "err2"; } catch e { print(e); }

// ۳. با مشخص کردن صریح نوع داده
try { throw "err3"; } catch (string e) { print(e); }

// ۴. مدل ناشناس (نادیده گرفتن مقدار خطا)
try { throw "err4"; } catch { print("خطا با موفقیت مهار شد"); }`}
        />
      </div>

      <h2 id="runtime-trapping" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        مهار خودکار خطاهای رانتایم توکار
      </h2>
      <p>
        خطاهای سیستمی رانتایم (نظیر تقسیم بر صفر یا اندیس اشتباه آرایه) به طور خودکار به استثناهای قابل دریافت با try/catch تبدیل می‌شوند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="trapping.persi"
          code={`// مهار خطای تقسیم بر صفر
try {
    int a = 10;
    int b = 0;
    int c = a / b;
} catch (err) {
    print("Trapped arithmetic error:", err);
}

// مهار خطای خروج از محدوده آرایه
try {
    int[] list = [1, 2, 3];
    int invalid = list[99];
} catch (err) {
    print("Trapped bounds error:", err);
}`}
          output={`Trapped arithmetic error: runtime error: division by zero
Trapped bounds error: runtime error: array index out of bounds: index 99 but length is 3`}
        />
      </div>

      <h2 id="unwinding" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        بازپیچی پشته فراخوانی (Call-Stack Unwinding)
      </h2>
      <p>
        هنگامی که خطایی در عمق توابع تو در تو رخ می‌دهد، پرسیکا پشته فراخوانی را پله‌پله بازپیچی کرده و حافظه‌های محلی را آزاد می‌کند تا به نزدیک‌ترین بلوک دربرگیرنده try برسد:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="stack_unwinding.persi"
          code={`def void level3() {
    print("In level 3: about to throw");
    throw "Fatal failure in level 3!";
    print("Unreachable");
}

def void level2() {
    print("In level 2");
    level3();
}

def void level1() {
    print("In level 1");
    level2();
}

try {
    level1();
} catch (failure) {
    print("Caught at top-level:", failure);
}`}
          output={`In level 1
In level 2
In level 3: about to throw
Caught at top-level: Fatal failure in level 3!`}
        />
      </div>

      <h2 id="rethrow" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        پرتاب مجدد خطا (Re-throwing)
      </h2>
      <p>
        بلوک‌های catch می‌توانند پس از ثبت لاگ یا انجام کار مقدماتی، استثنا را مجدداً به لایه‌های بیرونی پرتاب کنند:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="rethrow.persi"
          code={`try {
    try {
        throw "Initial trigger";
    } catch (inner) {
        print("Logged in inner handler:", inner);
        throw inner; // پرتاب مجدد به بلاک بیرونی
    }
} catch (outer) {
    print("Handled finally in outer block:", outer);
}`}
          output={`Logged in inner handler: Initial trigger
Handled finally in outer block: Initial trigger`}
        />
      </div>
    </DocsLayout>
  );
}
