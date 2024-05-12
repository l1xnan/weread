if (window.location.origin === "https://weread.qq.com") {
  console.log("hello world from js init script");

  window.__MY_CUSTOM_PROPERTY__ = { foo: "bar" };
}

function getCurrentMaxWidth(element) {
  let currentValue = window.getComputedStyle(element).maxWidth;
  currentValue = currentValue.substring(0, currentValue.indexOf("px"));
  currentValue = parseInt(currentValue);
  return currentValue;
}

function changeWidth(increse) {
  const step = 100;
  const item1 = document.querySelector(".readerContent .app_content");
  const item2 = document.querySelector(".readerTopBar");
  const currentValue = getCurrentMaxWidth(item1);
  const changedValue = currentValue + (increse ? 1 : -1) * step;

  item1.style["max-width"] = changedValue + "px";
  item2.style["max-width"] = changedValue + "px";
  const event = new Event("resize");
  window.dispatchEvent(event);
}

window.addEventListener("load", (_event) => {
  console.log("execute onload...");

  // 添加内容
  const btn1 = `
  <button id='lv-button1' title="加宽" class='readerControls_item extra-item widthIncrease' style='color:#6a6c6c;cursor:pointer;'>
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"  fill="none"  stroke="currentColor" stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-viewport-wide">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M10 12h-7l3 -3m0 6l-3 -3" /><path d="M14 12h7l-3 -3m0 6l3 -3" /><path d="M3 6v-3h18v3" /><path d="M3 18v3h18v-3" />
    </svg>
  </button>
  <button id='lv-button2' title="减宽" class='readerControls_item extra-item widthDecrease' style='color:#6a6c6c;cursor:pointer;'>
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-viewport-narrow">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M3 12h7l-3 -3m0 6l3 -3" /><path d="M21 12h-7l3 -3m0 6l-3 -3" /><path d="M9 6v-3h6v3" /><path d="M9 18v3h6v-3" />
    </svg>
  </button>

  <button id='autoScroll' title="滚动" class='readerControls_item extra-item autoScroll' style='color:#6a6c6c;cursor:pointer;font-size:14px;'>
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-square-chevrons-down">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M15 8l-3 3l-3 -3" /><path d="M15 13l-3 3l-3 -3" /><path d="M3 5a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2v-14z" />
    </svg>
  </button>
  <button id='stopScroll' title="停止" class='readerControls_item extra-item stopScroll' style='color:#6a6c6c;cursor:pointer;'>
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-player-pause">
      <path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M6 5m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z" /><path d="M14 5m0 1a1 1 0 0 1 1 -1h2a1 1 0 0 1 1 1v12a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1z" />
    </svg>
  </button>
  `;

  document
    .querySelector(".readerControls")
    ?.insertAdjacentHTML("beforeend", btn1);

  // 添加监听
  document
    .getElementById("lv-button1")
    ?.addEventListener("click", () => changeWidth(true));
  document
    .getElementById("lv-button2")
    ?.addEventListener("click", () => changeWidth(false));

  let num = 1;
  document.querySelector(".autoScroll")?.addEventListener("click", () => {
    num++;
    if (num > 10) {
      num = 1;
    }
    autoScroll();
    document.querySelector(".autoScroll").innerHTML = `×${num}`;
  });

  // 下划隐藏顶栏，上划显示顶栏
  let windowTop = 0;
  window.addEventListener("scroll", function () {
    let scrollS = window.scrollY;
    let selBtn = document.querySelector(".readerTopBar");
    const readerControl = document.querySelector(".readerControls");
    readerControl.addEventListener("mouseenter", () => {
      readerControl.style.opacity = 1;
    });

    readerControl.addEventListener("mouseleave", () => {
      readerControl.style.opacity = 0;
    });

    if (scrollS >= windowTop) {
      // 上划显示
      selBtn.style.opacity = 0;
      windowTop = scrollS;
    } else {
      // 下滑隐藏
      selBtn.style.opacity = 1;
      windowTop = scrollS;
    }
  });
  console.log("pre count =", document.querySelectorAll("pre").length);
  document.querySelectorAll("pre").forEach((elem) => {
    // 获取当前的 transform 属性值
    const transform = elem.style.transform;
    // 正则表达式匹配 translate 中的第一个值
    // translate(36px, 2423px) -> translate(0px, 2423px)
    const regex = /translate\((-?\d+px),/;
    // 替换为 0px
    elem.style.transform = transform.replace(regex, "translate(0px,");
    elem.style.width = "auto";
  });
});

// 滑动屏幕，滚至页面底部
function autoScroll() {
  let distance = 1;
  let timer = setInterval(() => {
    let totalHeight = document.documentElement.scrollTop;
    let scrollHeight = document.body.scrollHeight;
    window.scrollBy(0, distance);
    totalHeight += distance;
    if (totalHeight >= scrollHeight) {
      clearInterval(timer);
    }
    document
      .querySelector(".stopScroll")
      .addEventListener("click", function () {
        num = 0;
        clearInterval(timer);
      });
  }, 20);
}
