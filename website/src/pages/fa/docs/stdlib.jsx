import React from 'react';
import DocsLayout from '../../../components/DocsLayout';
import CodeBlock from '../../../components/CodeBlock';
import { Cpu, Layers, Sparkles } from 'lucide-react';

export default function StdlibPageFa() {
  const meta = {
    title: 'کتابخانه استاندارد و پیش‌درآمد (Auto-Prelude)',
    category: 'کتابخانه استاندارد',
    description: 'کتابخانه استاندارد خودمیزبان پرسیکا که ۱۰۰٪ با خود زبان پرسیکا نوشته شده و ساختارهای داده همیشه در دسترس پیش‌درآمد (Prelude).',
  };

  const toc = [
    { title: 'کتابخانه استاندارد خودمیزبان', id: 'self-hosted', level: 2 },
    { title: 'پیش‌درآمد خودکار (Auto-Prelude)', id: 'prelude', level: 2 },
    { title: 'ساختارهای Stack و Queue', id: 'stack-queue', level: 2 },
    { title: 'ساختارهای Set و StringBuilder', id: 'set-sb', level: 2 },
    { title: 'کانتینرهای Result و Option', id: 'result-option', level: 2 },
    { title: 'ماژول ریاضی استاندارد (std.math)', id: 'math-module', level: 2 },
    { title: 'ماژول کالکشن‌ها (std.collections)', id: 'collections-module', level: 2 },
  ];

  return (
    <DocsLayout meta={meta} toc={toc}>
      <h2 id="self-hosted" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-8">
        کتابخانه استاندارد خودمیزبان
      </h2>
      <p>
        پرسیکا به اصل <strong>Dogfooding</strong> پایبند است: تمامی ساختارهای داده پیشرفته و توابع کمکی سطح بالا مستقیماً با خود زبان پرسیکا پیاده‌سازی شده‌اند (در مسیر <code className="text-emerald-400">src/stdlib/</code>) و درون باینری کامپایلر قرار گرفته‌اند، بدون هیچ سربار اضافی در زمان اجرا.
      </p>

      <h2 id="prelude" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        پیش‌درآمد خودکار (Auto-Prelude)
      </h2>
      <p>
        پیش‌درآمد یا <strong>Prelude</strong> مجموعه‌ای از ساختارها و توابع ضروری است که در <strong>تمام برنامه‌های پرسیکا به طور پیش‌فرض و بدون نیاز به import</strong> در دسترس شما هستند:
      </p>

      <h2 id="stack-queue" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ساختارهای `Stack` و `Queue`
      </h2>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="stack_queue.persi"
          code={`// ۱. پشته (Stack: ورود آخر، خروج اول LIFO)
Stack stack = Stack();
stack.push("Layer 1");
stack.push("Layer 2");
stack.push("Layer 3");

print("Popped item:", stack.pop());
print("Top item (peek):", stack.peek());
print("Stack size:", stack.len());

// ۲. صف (Queue: ورود اول، خروج اول FIFO)
Queue queue = Queue();
queue.enqueue("Customer 1");
queue.enqueue("Customer 2");

print("Served first:", queue.dequeue());
print("Remaining in queue:", queue.len());`}
          output={`Popped item: Layer 3
Top item (peek): Layer 2
Stack size: 2
Served first: Customer 1
Remaining in queue: 1`}
        />
      </div>

      <h2 id="set-sb" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ساختارهای `Set` و `StringBuilder`
      </h2>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="set_sb.persi"
          code={`// ۱. مجموعه یکتا (Set: نادیده گرفتن موارد تکراری)
Set unique_ids = Set();
unique_ids.add("USR-101");
unique_ids.add("USR-102");
unique_ids.add("USR-101"); // مقدار تکراری اضافه نمی‌شود

print("Set size:", unique_ids.len());
print("Contains USR-102?", unique_ids.contains("USR-102"));

// ۲. سازنده رشته کارآمد (StringBuilder)
StringBuilder sb = StringBuilder();
sb.append_str("SELECT * FROM users ");
sb.append_str("WHERE status = 'active' ");
sb.append_line("ORDER BY id DESC;");

print("Constructed SQL:");
print(sb.to_string());`}
          output={`Set size: 2
Contains USR-102? true
Constructed SQL:
SELECT * FROM users WHERE status = 'active' ORDER BY id DESC;`}
        />
      </div>

      <h2 id="result-option" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        کانتینرهای `Result` و `Option`
      </h2>
      <p>
        الهام‌گرفته از سیستم تایپ‌های فانکشنال زبان Rust برای مدیریت تمیز خطا و مقادیر تهی:
      </p>

      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="result_option.persi"
          code={`// تقسیم امن با ساختار Result
Result safe_divide(int a, int b) {
    if b == 0 {
        return err("امکان تقسیم بر صفر وجود ندارد");
    }
    return ok(a / b);
}

Result res = safe_divide(100, 5);
if res.is_ok {
    print("Computation succeeded:", res.value);
} else {
    print("Computation failed:", res.error);
}`}
          output={`Computation succeeded: 20`}
        />
      </div>

      <h2 id="math-module" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ماژول ریاضی استاندارد (`std.math`)
      </h2>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="math_demo.persi"
          code={`from std.math import sqrt, pow, gcd, is_prime;

print("Square root of 81.0:", sqrt(81.0));
print("2 raised to power 8:", pow(2.0, 8.0));
print("Greatest common divisor of 54 and 24:", gcd(54, 24));
print("Is 97 prime?", is_prime(97));`}
          output={`Square root of 81.0: 9
2 raised to power 8: 256
Greatest common divisor of 54 and 24: 6
Is 97 prime? true`}
        />
      </div>

      <h2 id="collections-module" className="text-2xl font-bold text-white border-b border-slate-800 pb-3 mt-12">
        ماژول کالکشن‌ها (`std.collections`)
      </h2>
      <div dir="ltr" className="text-left">
        <CodeBlock
          language="persica"
          filename="algorithms_demo.persi"
          code={`from std.collections import zip, chunks, binary_search;

int[] numbers = [10, 20, 30, 40, 50, 60];

// تقسیم آرایه به بسته‌های ۲ عضوی
print("Chunks of 2:", chunks(numbers, 2));

// جستجوی دودویی سریع در آرایه مرتب
print("Index of 40 via binary search:", binary_search(numbers, 40));`}
          output={`Chunks of 2: [[10, 20], [30, 40], [50, 60]]
Index of 40 via binary search: 3`}
        />
      </div>
    </DocsLayout>
  );
}
