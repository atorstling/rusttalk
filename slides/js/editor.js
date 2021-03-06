window.addEventListener('load', function () {
	function showResult(resultArea, resultText) {
		if (resultText === undefined) {
			resultArea.style.display = "none";

			return;
		}

		resultArea.style.display = "block";
		resultArea.textContent = resultText;
	}

	function showResult2(compileArea, resultArea, compileText, resultText) {
    if (compileText === undefined) {
      compileArea.style.display = "none";
    } else {
      compileArea.style.display = "block";
      compileArea.textContent = compileText;
    }


		if (resultText === undefined) {
			resultArea.style.display = "none";
		} else {
		  resultArea.style.display = "block";
	  	resultArea.textContent = resultText;
    }

	}

	function execCode(editor, compileArea, resultArea) {
                var cod = editor.getValue();
		var req = new XMLHttpRequest();
                var isTest = cod.includes("#[test]");
/*
		var payload = {
			version: "stable",
			optimize: "0",
			code: cod,
			test: isTest
		};
		req.open('POST', "https://play.rust-lang.org/evaluate.json", true);

    */
/*
                var payload = {
			channel: "stable",
                        code: cod,
                        crateType: "bin",
                        mode: "debug",
			test: false
		};
		req.open('POST', "http://play.integer32.com/execute", true);
*/
                var payload = {
			channel: "nightly",
                        code: cod,
                        crateType: "bin",
                        mode: "debug",
			tests: isTest
		};
		req.open('POST', "http://localhost:4711/execute", true);

		req.onload = function(e) {
			if (req.readyState !== 4) {
				return;
			}
			if (req.status === 200) {
				var response = JSON.parse(req.response);
        //For old playground
        //showResult(resultArea, response.result);
        //For new playground
				//showResult(resultArea, response.stderr + response.stdout);
				showResult2(compileArea, resultArea, response.stderr, response.stdout);
				//showResult(resultArea, response.stdout);
			} else {
				showResult2(compileArea, resultArea,
					"Request failed with code: " + req.status, undefined);
			}
		};
		req.onerror = function(e) {
                                console.log(e);
				showResult2(compileArea, resultArea,
					"Failed to connect to the Playpen server: " + e, undefined);
		}
		req.setRequestHeader("Content-Type", "application/json");
		req.send(JSON.stringify(payload));
		showResult2(compileArea, resultArea, "Please wait...", undefined);
	}

function createElements(code) {
		//Create the div for Ace editor
		var div = document.createElement("div");
		div.style.width = "100%";
		code.parentNode.insertBefore(div, code);

		//Create the editor
		var editor = ace.edit(div);
		editor.$blockScrolling = Infinity;
		editor.setValue(code.textContent.trim(), -1);
		editor.setTheme("ace/theme/tomorrow");
		editor.getSession().setMode("ace/mode/rust");
		editor.setFontSize(24);
		editor.setOptions({ maxLines: Infinity });

		//The result area
		var resultArea = document.createElement("code");
    resultArea.setAttribute("class", "result");
		resultArea.style.display = "none"; //Hide
		code.parentNode.insertBefore(resultArea, code);

    //The compile area
    var compileArea = document.createElement("code");
    compileArea.setAttribute("class", "compile");
    compileArea.style.display = "none";
    code.parentNode.insertBefore(compileArea, resultArea);

		//The reset code button
		var resetBtn = document.createElement("button");
		resetBtn.innerHTML = "Reset";
		resetBtn.className = "reset-button";
		resetBtn.addEventListener("click", function() {
			editor.setValue(code.textContent.trim(), -1);
			showResult2(compileArea, resultArea);
		});
		code.parentNode.insertBefore(resetBtn, compileArea);

		//The run button
		var runBtn = document.createElement("button");
		runBtn.innerHTML = "Run";
		runBtn.className = "run-button";
		//Accesskey won't work since there are many buttons with the same accesskey,
		// since all slides are on the same html page.
		runBtn.accessKey = "u";
		//runBtn.addEventListener("click", function() {
	//		execCode(editor, resultArea);
	//	});
		runBtn.onclick = function() { execCode(editor, compileArea, resultArea); }
		code.parentNode.insertBefore(runBtn, resetBtn);
}

    function initEditor() {
		var list = document.querySelectorAll("script[language='rust']");

		for (var i = 0; i < list.length; ++i) {
				var code = list[i];

				createElements(code);
		}
	}

	initEditor();
});
