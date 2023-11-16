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
  let changedValue;
  if (increse) {
    changedValue = currentValue + step;
  } else {
    changedValue = currentValue - step;
  }
  item1.style["max-width"] = changedValue + "px";
  item2.style["max-width"] = changedValue + "px";
  const myEvent = new Event("resize");
  window.dispatchEvent(myEvent);
}

window.addEventListener("load", (_event) => {
  console.log("execute onload...");

  // 添加内容
  const btn1 = `
  <button id='lv-button1' class='readerControls_item widthIncrease' style='color:#6a6c6c;cursor:pointer;'>加宽</button>
  <button id='lv-button2' class='readerControls_item widthDecrease' style='color:#6a6c6c;cursor:pointer;'>减宽</button>

  <button id='autoScroll' class='readerControls_item autoScroll' style='color:#6a6c6c;cursor:pointer;'>滚动X1</button>
  <button id='stopScroll' class='readerControls_item stopScroll' style='color:#6a6c6c;cursor:pointer;'>停止</button>
  `;
  
  document
    .querySelector(".readerControls")
    .insertAdjacentHTML("beforeend", btn1);

  // 添加监听
  document
    .getElementById("lv-button1")
    .addEventListener("click", () => changeWidth(true));
  document
    .getElementById("lv-button2")
    .addEventListener("click", () => changeWidth(false));

  let num = 1;
  document.querySelector(".autoScroll").addEventListener("click", () => {
    num++;
    if (num > 10) {
      num = 1;
    }
    autoScroll();
    document.querySelector(".autoScroll").innerHTML = `播放×${num}`;
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
