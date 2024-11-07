use std::env::var;
use rand::seq::SliceRandom;

// from grammar import Grammar
// from svg_tags import _SVG_TYPES => Is now an array
// from html_tags import _HTML_TYPES => Is now an array
// from mathml_tags import _MATHML_TYPES => Is now an array

pub struct HtmlVar {
    pub name: String,
    pub type_: String,
}

pub struct Context {
    pub htmlvarctr: u32,
    pub svgvarctr: u32,
    pub mathmlvarctr: u32,
    pub htmlvars: Vec<HtmlVar>,
    pub htmlvargen: String,
}

/// The `generate_html_elements` functions takes two arguments: a Context and a u32
/// It handles the creation of html elements based on HTML_TYPE tags

pub fn generate_html_elements(ctx: &mut Context, n: u32) {
    let mut rng = rand::thread_rng(); // Random number generation

    for _ in 0..n {
        let &(html_tag, html_type) = HTML_TYPES.choose(&mut rng).unwrap(); // Choosing random html_tag and html_type
        ctx.htmlvarctr += 1;
        let var_name = format!("htmlvar{:05}", ctx.htmlvarctr);

        ctx.htmlvars.push(HtmlVar {
            name: var_name.clone(),
            type_: html_type.to_string(),
        }); // Pushes values to struct HtmlVar

        ctx.htmlvargen.push_str(&format!("/* newvar{{{var_name}:{html_type}}} */ var {var_name} = document.createElement(\"{html_tag}\"); // {html_type}\n"));
        // Format output
    }
}

/// The `add_html_ids` function takes a regex of html id and a context as input
/// Its role is to add html id
fn add_html_ids(matchobj: &regex::Captures, ctx: &mut Context) -> String {
    let tagname = &matchobj[0][1..matchobj[0].len() - 1]; // Extract tag name without '<' and trailing space

    // Define a closure to handle the common logic
    let mut process_tag = |counter: &mut u32, prefix: &str| {
        *counter += 1;
        let var_name = format!("{prefix}var{:05}", *counter);
        ctx.htmlvars.push(HtmlVar {
            name: var_name.clone(),
            type_: tagname.to_string(),
        });
        ctx.htmlvargen.push_str(&format!("/* newvar{{{var_name}}}:{tagname} */ var {var_name} = document.getElementById(\"{var_name}\"); // {tagname}\n"));
        matchobj[0].to_string() + &format!("id=\"' + {var_name} + '\"")
    };

    if HTML_TYPES.iter().any(|tagname| HTML_TYPES.contains(tagname)) {
        process_tag(&mut ctx.htmlvarctr, "html")
    } else if SVG_TYPES.iter().any(|tagname| SVG_TYPES.contains(tagname)) {
        process_tag(&mut ctx.svgvarctr, "svg")
    } else if MATHML_TYPES.iter().any(|tagname| MATHML_TYPES.contains(tagname)) {
        process_tag(&mut ctx.mathmlvarctr, "mathml")
    } else {
        matchobj[0].to_string()
    }
}

// fn generate_function_body(jsgrammar, htmlctx, num_lines)
/*
    js = ''
    js += 'var fuzzervars = {};\n\n'
    js += "SetVariable(fuzzervars, window, 'Window');\nSetVariable(fuzzervars, document, 'Document');\nSetVariable(fuzzervars, document.body.firstChild, 'Element');\n\n"
    js += '//beginjs\n'
    js += htmlctx['htmlvargen']
    js += jsgrammar._generate_code(num_lines, htmlctx['htmlvars'])
    js += '\n//endjs\n'
    js += 'var fuzzervars = {};\nfreememory()\n'
    return js
*/

//fn check_grammar(grammar)
/*
    """Checks if grammar has errors and if so outputs them.
    Args:
      grammar: The grammar to check.
    """

    for rule in grammar._all_rules:
        for part in rule['parts']:
            if part['type'] == 'text':
                continue
            tagname = part['tagname']
            # print tagname
            if tagname not in grammar._creators:
                print('No creators for type ' + tagname)
*/

// fn generate_new_sample(template, htmlgrammar, cssgrammar, jsgrammar)
/*
    """Parses grammar rules from string.
    Args:
      template: A template string.
      htmlgrammar: Grammar for generating HTML code.
      cssgrammar: Grammar for generating CSS code.
      jsgrammar: Grammar for generating JS code.
    Returns:
      A string containing sample data.
    """

    result = template

    css = cssgrammar.generate_symbol('rules')
    html = htmlgrammar.generate_symbol('bodyelements')

    htmlctx = {
        'htmlvars': [],
        'htmlvarctr': 0,
        'svgvarctr': 0,
        'mathmlvarctr': 0,
        'htmlvargen': ''
    }
    html = re.sub(
        r'<[a-zA-Z0-9_-]+ ',
        lambda match: add_html_ids(match, htmlctx),
        html
    )
    generate_html_elements(htmlctx, _N_ADDITIONAL_HTMLVARS)

    result = result.replace('<cssfuzzer>', css)
    result = result.replace('<htmlfuzzer>', html)

    handlers = False
    while '<jsfuzzer>' in result:
        numlines = _N_MAIN_LINES
        if handlers:
            numlines = _N_EVENTHANDLER_LINES
        else:
            handlers = True
        result = result.replace(
            '<jsfuzzer>',
            generate_function_body(jsgrammar, htmlctx, numlines),
            1
        )

    return result
*/

// fn generate_samples(template, outfiles)
/*
    """Generates a set of samples and writes them to the output files.
    Args:
      grammar_dir: directory to load grammar files from.
      outfiles: A list of output filenames.
    """

    grammar_dir = os.path.join(os.path.dirname(__file__), 'rules')
    htmlgrammar = Grammar()

    err = htmlgrammar.parse_from_file(os.path.join(grammar_dir, 'html.txt'))
    # CheckGrammar(htmlgrammar)
    if err > 0:
        print('There were errors parsing html grammar')
        return

    cssgrammar = Grammar()
    err = cssgrammar.parse_from_file(os.path.join(grammar_dir ,'css.txt'))
    # CheckGrammar(cssgrammar)
    if err > 0:
        print('There were errors parsing css grammar')
        return

    jsgrammar = Grammar()
    err = jsgrammar.parse_from_file(os.path.join(grammar_dir,'js.txt'))
    # CheckGrammar(jsgrammar)
    if err > 0:
        print('There were errors parsing js grammar')
        return

    # JS and HTML grammar need access to CSS grammar.
    # Add it as import
    htmlgrammar.add_import('cssgrammar', cssgrammar)
    jsgrammar.add_import('cssgrammar', cssgrammar)

    for outfile in outfiles:
        result = generate_new_sample(template, htmlgrammar, cssgrammar, jsgrammar)
        if result is not None:
            print('Writing a sample to ' + outfile)
            try:
                with open(outfile, 'w') as f:
                    f.write(result)
            except IOError:
                print('Error writing to output')
*/

// fn get_argument_parser()
/*

    parser = argparse.ArgumentParser(description="DOMATO (A DOM FUZZER)")

    parser.add_argument("-f", "--file",
    help="File name which is to be generated in the same directory")

    parser.add_argument('-o', '--output_dir', type=str,
                    help='The output directory to put the generated files in')

    parser.add_argument('-n', '--no_of_files', type=int,
                    help='number of files to be generated')

    parser.add_argument('-t', '--template', type=Path, default=(Path(__file__).parent).joinpath('template.html'),
                    help='template file you want to use')
    return parser
*/

/// Defining HTML_TYPES
static HTML_TYPES: &[(&str, &str)] = &[
    ("a", "HTMLAnchorElement"),
    ("abbr", "HTMLUnknownElement"),
    ("acronym", "HTMLUnknownElement"),
    ("address", "HTMLUnknownElement"),
    ("applet", "HTMLUnknownElement"),
    ("area", "HTMLAreaElement"),
    ("article", "HTMLUnknownElement"),
    ("aside", "HTMLUnknownElement"),
    ("audio", "HTMLAudioElement"),
    ("b", "HTMLUnknownElement"),
    ("base", "HTMLBaseElement"),
    ("basefont", "HTMLUnknownElement"),
    ("bdi", "HTMLUnknownElement"),
    ("bdo", "HTMLUnknownElement"),
    ("bgsound", "HTMLUnknownElement"),
    ("big", "HTMLUnknownElement"),
    ("blockquote", "HTMLUnknownElement"),
    ("br", "HTMLBRElement"),
    ("button", "HTMLButtonElement"),
    ("canvas", "HTMLCanvasElement"),
    ("caption", "HTMLTableCaptionElement"),
    ("center", "HTMLUnknownElement"),
    ("cite", "HTMLUnknownElement"),
    ("code", "HTMLUnknownElement"),
    ("col", "HTMLTableColElement"),
    ("colgroup", "HTMLUnknownElement"),
    ("command", "HTMLUnknownElement"),
    ("content", "HTMLContentElement"),
    ("data", "HTMLDataElement"),
    ("datalist", "HTMLDataListElement"),
    ("dd", "HTMLUnknownElement"),
    ("del", "HTMLModElement"),
    ("details", "HTMLDetailsElement"),
    ("dfn", "HTMLUnknownElement"),
    ("dialog", "HTMLDialogElement"),
    ("dir", "HTMLDirectoryElement"),
    ("div", "HTMLDivElement"),
    ("dl", "HTMLDListElement"),
    ("dt", "HTMLUnknownElement"),
    ("em", "HTMLUnknownElement"),
    ("embed", "HTMLEmbedElement"),
    ("fieldset", "HTMLFieldSetElement"),
    ("figcaption", "HTMLUnknownElement"),
    ("figure", "HTMLUnknownElement"),
    ("font", "HTMLFontElement"),
    ("footer", "HTMLUnknownElement"),
    ("form", "HTMLFormElement"),
    ("frame", "HTMLFrameElement"),
    ("frameset", "HTMLFrameSetElement"),
    ("h1", "HTMLHeadingElement"),
    ("h2", "HTMLHeadingElement"),
    ("h3", "HTMLHeadingElement"),
    ("h4", "HTMLHeadingElement"),
    ("h5", "HTMLHeadingElement"),
    ("h6", "HTMLHeadingElement"),
    ("header", "HTMLUnknownElement"),
    ("hgroup", "HTMLUnknownElement"),
    ("hr", "HTMLHRElement"),
    ("i", "HTMLUnknownElement"),
    ("iframe", "HTMLIFrameElement"),
    ("image", "HTMLImageElement"),
    ("img", "HTMLImageElement"),
    ("input", "HTMLInputElement"),
    ("ins", "HTMLModElement"),
    ("isindex", "HTMLUnknownElement"),
    ("kbd", "HTMLUnknownElement"),
    ("keygen", "HTMLKeygenElement"),
    ("label", "HTMLLabelElement"),
    ("layer", "HTMLUnknownElement"),
    ("legend", "HTMLLegendElement"),
    ("li", "HTMLLIElement"),
    ("link", "HTMLLinkElement"),
    ("listing", "HTMLUnknownElement"),
    ("main", "HTMLUnknownElement"),
    ("map", "HTMLMapElement"),
    ("mark", "HTMLUnknownElement"),
    ("marquee", "HTMLMarqueeElement"),
    ("menu", "HTMLMenuElement"),
    ("menuitem", "HTMLMenuItemElement"),
    ("meta", "HTMLMetaElement"),
    ("meter", "HTMLMeterElement"),
    ("nav", "HTMLUnknownElement"),
    ("nobr", "HTMLUnknownElement"),
    ("noembed", "HTMLUnknownElement"),
    ("noframes", "HTMLUnknownElement"),
    ("nolayer", "HTMLUnknownElement"),
    ("noscript", "HTMLUnknownElement"),
    ("object", "HTMLObjectElement"),
    ("ol", "HTMLOListElement"),
    ("optgroup", "HTMLOptGroupElement"),
    ("option", "HTMLOptionElement"),
    ("output", "HTMLOutputElement"),
    ("p", "HTMLParagraphElement"),
    ("param", "HTMLParamElement"),
    ("picture", "HTMLPictureElement"),
    ("plaintext", "HTMLUnknownElement"),
    ("pre", "HTMLPreElement"),
    ("progress", "HTMLProgressElement"),
    ("q", "HTMLQuoteElement"),
    ("rp", "HTMLUnknownElement"),
    ("rt", "HTMLUnknownElement"),
    ("ruby", "HTMLUnknownElement"),
    ("s", "HTMLUnknownElement"),
    ("samp", "HTMLUnknownElement"),
    ("section", "HTMLUnknownElement"),
    ("select", "HTMLSelectElement"),
    ("shadow", "HTMLShadowElement"),
    ("small", "HTMLUnknownElement"),
    ("source", "HTMLSourceElement"),
    ("span", "HTMLSpanElement"),
    ("strike", "HTMLUnknownElement"),
    ("strong", "HTMLUnknownElement"),
    ("style", "HTMLStyleElement"),
    ("sub", "HTMLUnknownElement"),
    ("summary", "HTMLUnknownElement"),
    ("sup", "HTMLUnknownElement"),
    ("table", "HTMLTableElement"),
    ("tbody", "HTMLTableSectionElement"),
    ("td", "HTMLUnknownElement"),
    ("template", "HTMLTemplateElement"),
    ("textarea", "HTMLTextAreaElement"),
    ("tfoot", "HTMLTableSectionElement"),
    ("th", "HTMLTableCellElement"),
    ("thead", "HTMLTableSectionElement"),
    ("time", "HTMLTimeElement"),
    ("title", "HTMLTitleElement"),
    ("tr", "HTMLTableRowElement"),
    ("track", "HTMLTrackElement"),
    ("tt", "HTMLUnknownElement"),
    ("u", "HTMLUnknownElement"),
    ("ul", "HTMLUListElement"),
    ("var", "HTMLUnknownElement"),
    ("video", "HTMLVideoElement"),
    ("wbr", "HTMLUnknownElement"),
    ("xmp", "HTMLUnknownElement"),
];

/// Defining SVG_TYPES
static SVG_TYPES: &[(&str, &str)] = &[
    ("a", "SVGAElement"),
    ("altGlyph", "SVGElement"),
    ("altGlyphDef", "SVGElement"),
    ("altGlyphItem", "SVGElement"),
    ("animate", "SVGAnimateElement"),
    ("animateColor", "SVGElement"),
    ("animateMotion", "SVGAnimateMotionElement"),
    ("animateTransform", "SVGAnimateTransformElement"),
    ("circle", "SVGCircleElement"),
    ("clipPath", "SVGClipPathElement"),
    ("color-profile", "SVGElement"),
    ("cursor", "SVGCursorElement"),
    ("defs", "SVGDefsElement"),
    ("desc", "SVGDescElement"),
    ("discard", "SVGDiscardElement"),
    ("ellipse", "SVGEllipseElement"),
    ("feBlend", "SVGFEBlendElement"),
    ("feColorMatrix", "SVGFEColorMatrixElement"),
    ("feComponentTransfer", "SVGFEComponentTransferElement"),
    ("feComposite", "SVGFECompositeElement"),
    ("feConvolveMatrix", "SVGFEConvolveMatrixElement"),
    ("feDiffuseLighting", "SVGFEDiffuseLightingElement"),
    ("feDisplacementMap", "SVGFEDisplacementMapElement"),
    ("feDistantLight", "SVGFEDistantLightElement"),
    ("feDropShadow", "SVGFEDropShadowElement"),
    ("feFlood", "SVGFEFloodElement"),
    ("feFuncA", "SVGFEFuncAElement"),
    ("feFuncB", "SVGFEFuncBElement"),
    ("feFuncG", "SVGFEFuncGElement"),
    ("feFuncR", "SVGFEFuncRElement"),
    ("feGaussianBlur", "SVGFEGaussianBlurElement"),
    ("feImage", "SVGFEImageElement"),
    ("feMerge", "SVGFEMergeElement"),
    ("feMergeNode", "SVGFEMergeNodeElement"),
    ("feMorphology", "SVGFEMorphologyElement"),
    ("feOffset", "SVGFEOffsetElement"),
    ("fePointLight", "SVGFEPointLightElement"),
    ("feSpecularLighting", "SVGFESpecularLightingElement"),
    ("feSpotLight", "SVGFESpotLightElement"),
    ("feTile", "SVGFETileElement"),
    ("feTurbulence", "SVGFETurbulenceElement"),
    ("filter", "SVGFilterElement"),
    ("font", "SVGElement"),
    ("font-face", "SVGElement"),
    ("font-face-format", "SVGElement"),
    ("font-face-name", "SVGElement"),
    ("font-face-src", "SVGElement"),
    ("font-face-uri", "SVGElement"),
    ("foreignObject", "SVGForeignObjectElement"),
    ("g", "SVGGElement"),
    ("glyph", "SVGElement"),
    ("glyphRef", "SVGElement"),
    ("hatch", "SVGElement"),
    ("hatchpath", "SVGElement"),
    ("hkern", "SVGElement"),
    ("image", "SVGImageElement"),
    ("line", "SVGLineElement"),
    ("linearGradient", "SVGLinearGradientElement"),
    ("marker", "SVGMarkerElement"),
    ("mask", "SVGMaskElement"),
    ("mesh", "SVGElement"),
    ("meshgradient", "SVGElement"),
    ("meshpatch", "SVGElement"),
    ("meshrow", "SVGElement"),
    ("metadata", "SVGMetadataElement"),
    ("missing-glyph", "SVGElement"),
    ("mpath", "SVGMPathElement"),
    ("path", "SVGPathElement"),
    ("pattern", "SVGPatternElement"),
    ("polygon", "SVGPolygonElement"),
    ("polyline", "SVGPolylineElement"),
    ("radialGradient", "SVGRadialGradientElement"),
    ("rect", "SVGRectElement"),
    ("set", "SVGSetElement"),
    ("svg", "SVGSVGElement"),
    ("solidcolor", "SVGElement"),
    ("stop", "SVGStopElement"),
    ("switch", "SVGSwitchElement"),
    ("symbol", "SVGSymbolElement"),
    ("text", "SVGTextElement"),
    ("textPath", "SVGTextPathElement"),
    ("title", "SVGTitleElement"),
    ("tref", "SVGElement"),
    ("tspan", "SVGTSpanElement"),
    ("unknown", "SVGElement"),
    ("use", "SVGUseElement"),
    ("view", "SVGViewElement"),
    ("vkern", "SVGElement"),
];

/// Defining MATHML_TYPES
static MATHML_TYPES: &[(&str, &str)] = &[
    ("annotation", "MathMLElement"),
    ("annotation-xml", "MathMLElement"),
    ("maction", "MathMLElement"),
    ("math", "MathMLElement"),
    ("merror", "MathMLElement"),
    ("mfrac", "MathMLElement"),
    ("mi", "MathMLElement"),
    ("mmultiscripts", "MathMLElement"),
    ("mn", "MathMLElement"),
    ("mo", "MathMLElement"),
    ("mover", "MathMLElement"),
    ("mpadded", "MathMLElement"),
    ("mphantom", "MathMLElement"),
    ("mprescripts", "MathMLElement"),
    ("mroot", "MathMLElement"),
    ("mrow", "MathMLElement"),
    ("ms", "MathMLElement"),
    ("mspace", "MathMLElement"),
    ("msqrt", "MathMLElement"),
    ("mstyle", "MathMLElement"),
    ("msub", "MathMLElement"),
    ("msubsup", "MathMLElement"),
    ("msup", "MathMLElement"),
    ("mtable", "MathMLElement"),
    ("mtd", "MathMLElement"),
    ("mtext", "MathMLElement"),
    ("mtr", "MathMLElement"),
    ("munder", "MathMLElement"),
    ("munderover", "MathMLElement"),
    ("none", "MathMLElement"),
    ("semantics", "MathMLelement"),
];
