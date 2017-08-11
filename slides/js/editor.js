window.addEventListener('load', function () {
	function showResult(resultArea, resultText) {
		if (resultText === undefined) {
			resultArea.style.display = "none";

			return;
		}

		resultArea.style.display = "block";
		resultArea.textContent = resultText;
	}

	function execCode(editor, resultArea) {
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
			tests: false
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
				showResult(resultArea, response.stderr + response.stdout);
				//showResult(resultArea, response.stdout);
			} else {
				showResult(resultArea,
					"Request failed with code: " + req.status);
			}
		};
		req.onerror = function(e) {
                                console.log(e);
				showResult(resultArea,
					"Failed to connect to the Playpen server: " + e);
		}
		req.setRequestHeader("Content-Type", "application/json");
		req.send(JSON.stringify(payload));
		showResult(resultArea, "Please wait...");
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
		resultArea.style.display = "none"; //Hide
		code.parentNode.insertBefore(resultArea, code);

		//The reset code button
		var resetBtn = document.createElement("button");
		resetBtn.innerHTML = "Reset";
		resetBtn.className = "reset-button";
		resetBtn.addEventListener("click", function() {
			editor.setValue(code.textContent.trim(), -1);
			showResult(resultArea);
		});
		code.parentNode.insertBefore(resetBtn, resultArea);

		//The run button
		var runBtn = document.createElement("button");
		runBtn.innerHTML = "Run";
		runBtn.className = "run-button";
		runBtn.accessKey = "r";
		//runBtn.addEventListener("click", function() {
	//		execCode(editor, resultArea);
	//	});
		runBtn.onclick = function() { execCode(editor, resultArea); }
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
