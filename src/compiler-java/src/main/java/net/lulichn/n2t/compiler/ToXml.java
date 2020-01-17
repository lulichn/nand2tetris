package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.ParserRuleContext;
import org.antlr.v4.runtime.tree.ParseTree;
import org.antlr.v4.runtime.tree.TerminalNodeImpl;
import org.w3c.dom.Document;
import org.w3c.dom.Node;

import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

import static net.lulichn.n2t.compiler.JackParser.*;

public class ToXml {
  static final List<String> keywords = Arrays.asList("class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else",
      "while", "return");

  static final List<Integer> parents = Arrays.asList(
      RULE_varDec,
      RULE_classVarDec,
      RULE_letStatement,
      RULE_ifStatement,
      RULE_whileStatement,
      RULE_doStatement,
      RULE_returnStatement,
      RULE_term,
      RULE_expression,
      RULE_statements,
      RULE_subroutineDec,
      RULE_subroutineBody,
      RULE_klass,
      RULE_parameterList,
      RULE_expressionList);

  static final List<Integer> temporary = Arrays.asList(
      RULE_varType,
      RULE_varList,
      RULE_type,
      RULE_subroutineKind,
      RULE_returnType,
      RULE_typedVar,
      RULE_className,
      RULE_subroutineName,
      RULE_varName,
      RULE_statement,
      RULE_arrayIndexing,
      RULE_elseClause,
      RULE_subroutineCall,
      RULE_qualifier,
      RULE_op,
      RULE_unaryOp,
      RULE_keywordConstant);

  private final Document doc;

  public ToXml(Document doc) {
    this.doc = doc;
  }

  public static Node klass(JackParser.KlassContext klass) throws ParserConfigurationException {
    var doc = createXMLDocument("root");
    var toXml = new ToXml(doc);
    var xml = new ToXml(doc).make(klass);
    return xml.get(0);
  }

  public static Document createXMLDocument(String root) throws ParserConfigurationException {
    DocumentBuilderFactory factory = DocumentBuilderFactory.newInstance();
    var builder = factory.newDocumentBuilder();

    var dom = builder.getDOMImplementation();
    return dom.createDocument("", root, null);
  }

  public List<Node> make(ParseTree tree) {
    if (tree instanceof JackParser.IdentifierContext) {
      return List.of(text("identifier", tree.getText()));
    }

    if (tree instanceof JackParser.StringConstantContext) {
      return List.of(text("stringConstant", tree.getText().substring(1, tree.getText().length() - 1)));
    }

    if (tree instanceof JackParser.IntegerConstantContext) {
      return List.of(text("integerConstant", tree.getText()));
    }

    if (tree instanceof TerminalNodeImpl) {
      var text = tree.getText();
      if (keywords.contains(text)) {
        return List.of(text("keyword", text));
      } else {
        return List.of(text("symbol", text));
      }
    }

    var list = new ArrayList<Node>();
    for (int id = 0; id < tree.getChildCount(); id++) {
      list.addAll(make(tree.getChild(id)));
    }

    var ruleIndex = ((ParserRuleContext) tree).getRuleIndex();

    // temp. not parent
    if (temporary.contains(ruleIndex)) {
      return list;
    }

    // container? parent
    String containerName;
    if (ruleIndex == RULE_klass) {
      containerName = "class";
    } else if (parents.contains(ruleIndex)) {
      containerName = ruleNames[ruleIndex];
    } else {
      throw new IllegalArgumentException(tree.getText());
    }

    var parent = doc.createElement(containerName);
    for (var c: list) {
      parent.appendChild(c);
    }

    return List.of(parent);
  }

  Node text(String name, String value) {
    var node = doc.createElement(name);
    var textNode = doc.createTextNode(" " + value + " ");
    node.appendChild(textNode);
    return node;
  }
}