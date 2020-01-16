package net.lulichn.n2t.compiler;

import org.antlr.v4.runtime.ParserRuleContext;
import org.antlr.v4.runtime.tree.ErrorNode;
import org.antlr.v4.runtime.tree.TerminalNode;

import java.util.ArrayList;
import java.util.List;

public class JackListenerImpl implements JackListener {
  List<String> test = new ArrayList<>();

  public List<String> test() {
    return this.test;
  }

  String keyword(String v) {
    return "<keyword> " + v + " </keyword>";
  }

  String symbol(String v) {
    return "<symbol> " + v + " </symbol>";
  }

  String identifier(String v) {
    return "<identifier> " + v + " </identifier>";
  }

  @Override
  public void enterKlass(JackParser.KlassContext ctx) {
    test.add("<class>");
  }

  @Override
  public void exitKlass(JackParser.KlassContext ctx) {
    test.add("</class>");
  }

  @Override
  public void enterClassVarDec(JackParser.ClassVarDecContext ctx) {

  }

  @Override
  public void exitClassVarDec(JackParser.ClassVarDecContext ctx) {

  }

  @Override
  public void enterVarType(JackParser.VarTypeContext ctx) {

  }

  @Override
  public void exitVarType(JackParser.VarTypeContext ctx) {

  }

  @Override
  public void enterVarList(JackParser.VarListContext ctx) {

  }

  @Override
  public void exitVarList(JackParser.VarListContext ctx) {

  }

  @Override
  public void enterType(JackParser.TypeContext ctx) {
    test.add(keyword(ctx.children.get(0).getText()));
  }

  @Override
  public void exitType(JackParser.TypeContext ctx) {

  }

  @Override
  public void enterSubroutineDec(JackParser.SubroutineDecContext ctx) {
    test.add("<subroutineDec>");
  }

  @Override
  public void exitSubroutineDec(JackParser.SubroutineDecContext ctx) {

  }

  @Override
  public void enterSubroutineKind(JackParser.SubroutineKindContext ctx) {
    test.add(keyword(ctx.children.get(0).getText()));
  }

  @Override
  public void exitSubroutineKind(JackParser.SubroutineKindContext ctx) {

  }

  @Override
  public void enterReturnType(JackParser.ReturnTypeContext ctx) {
    test.add(keyword(ctx.children.get(0).getText()));
  }

  @Override
  public void exitReturnType(JackParser.ReturnTypeContext ctx) {

  }

  @Override
  public void enterParameterList(JackParser.ParameterListContext ctx) {
    test.add("<parameterList>");
  }

  @Override
  public void exitParameterList(JackParser.ParameterListContext ctx) {
    test.add("</parameterList>");
  }

  @Override
  public void enterTypedVar(JackParser.TypedVarContext ctx) {

  }

  @Override
  public void exitTypedVar(JackParser.TypedVarContext ctx) {

  }

  @Override
  public void enterSubroutineBody(JackParser.SubroutineBodyContext ctx) {
    test.add("<subroutineBody>");
  }

  @Override
  public void exitSubroutineBody(JackParser.SubroutineBodyContext ctx) {

  }

  @Override
  public void enterVarDec(JackParser.VarDecContext ctx) {
    test.add("<varDec>");
  }

  @Override
  public void exitVarDec(JackParser.VarDecContext ctx) {
    test.add("</varDec>");
  }

  @Override
  public void enterClassName(JackParser.ClassNameContext ctx) {
    test.add(identifier(ctx.children.get(0).getText()));
  }

  @Override
  public void exitClassName(JackParser.ClassNameContext ctx) {

  }

  @Override
  public void enterSubroutineName(JackParser.SubroutineNameContext ctx) {
    test.add(identifier(ctx.children.get(0).getText()));
  }

  @Override
  public void exitSubroutineName(JackParser.SubroutineNameContext ctx) {

  }

  @Override
  public void enterVarName(JackParser.VarNameContext ctx) {
    test.add(identifier(ctx.children.get(0).getText()));
  }

  @Override
  public void exitVarName(JackParser.VarNameContext ctx) {

  }

  @Override
  public void enterStatements(JackParser.StatementsContext ctx) {
    test.add("<statements>");
  }

  @Override
  public void exitStatements(JackParser.StatementsContext ctx) {
    test.add("</statements>");
  }

  @Override
  public void enterStatement(JackParser.StatementContext ctx) {

  }

  @Override
  public void exitStatement(JackParser.StatementContext ctx) {

  }

  @Override
  public void enterLetStatement(JackParser.LetStatementContext ctx) {
    test.add("<letStatement>");
  }

  @Override
  public void exitLetStatement(JackParser.LetStatementContext ctx) {
    test.add("</letStatement>");
  }

  @Override
  public void enterArrayIndexing(JackParser.ArrayIndexingContext ctx) {

  }

  @Override
  public void exitArrayIndexing(JackParser.ArrayIndexingContext ctx) {

  }

  @Override
  public void enterIfStatement(JackParser.IfStatementContext ctx) {

  }

  @Override
  public void exitIfStatement(JackParser.IfStatementContext ctx) {

  }

  @Override
  public void enterElseClause(JackParser.ElseClauseContext ctx) {

  }

  @Override
  public void exitElseClause(JackParser.ElseClauseContext ctx) {

  }

  @Override
  public void enterWhileStatement(JackParser.WhileStatementContext ctx) {
    test.add("<whileStatement>");
    test.add(keyword(ctx.children.get(0).getText()));
  }

  @Override
  public void exitWhileStatement(JackParser.WhileStatementContext ctx) {
    test.add("</whileStatement>");
  }

  @Override
  public void enterDoStatement(JackParser.DoStatementContext ctx) {

  }

  @Override
  public void exitDoStatement(JackParser.DoStatementContext ctx) {

  }

  @Override
  public void enterReturnStatement(JackParser.ReturnStatementContext ctx) {

  }

  @Override
  public void exitReturnStatement(JackParser.ReturnStatementContext ctx) {

  }

  @Override
  public void enterExpression(JackParser.ExpressionContext ctx) {
    test.add("<expression>");
  }

  @Override
  public void exitExpression(JackParser.ExpressionContext ctx) {
    test.add("</expression>");
  }

  @Override
  public void enterTerm(JackParser.TermContext ctx) {
    test.add("<term>");
  }

  @Override
  public void exitTerm(JackParser.TermContext ctx) {
    test.add("</term>");
  }

  @Override
  public void enterSubroutineCall(JackParser.SubroutineCallContext ctx) {

  }

  @Override
  public void exitSubroutineCall(JackParser.SubroutineCallContext ctx) {

  }

  @Override
  public void enterQualifier(JackParser.QualifierContext ctx) {

  }

  @Override
  public void exitQualifier(JackParser.QualifierContext ctx) {

  }

  @Override
  public void enterExpressionList(JackParser.ExpressionListContext ctx) {

  }

  @Override
  public void exitExpressionList(JackParser.ExpressionListContext ctx) {

  }

  @Override
  public void enterIntegerConstant(JackParser.IntegerConstantContext ctx) {
    test.add("<integerConstant> " + ctx.children.get(0) + " </integerConstant>");
  }

  @Override
  public void exitIntegerConstant(JackParser.IntegerConstantContext ctx) {

  }

  @Override
  public void enterStringConstant(JackParser.StringConstantContext ctx) {
    test.add("<stringConstant> " + ctx.children.get(0) + " </stringConstant>");
  }

  @Override
  public void exitStringConstant(JackParser.StringConstantContext ctx) {

  }

  @Override
  public void enterOp(JackParser.OpContext ctx) {

  }

  @Override
  public void exitOp(JackParser.OpContext ctx) {

  }

  @Override
  public void enterUnaryOp(JackParser.UnaryOpContext ctx) {

  }

  @Override
  public void exitUnaryOp(JackParser.UnaryOpContext ctx) {

  }

  @Override
  public void enterKeywordConstant(JackParser.KeywordConstantContext ctx) {

  }

  @Override
  public void exitKeywordConstant(JackParser.KeywordConstantContext ctx) {

  }

  @Override
  public void visitTerminal(TerminalNode node) {

  }

  @Override
  public void visitErrorNode(ErrorNode node) {

  }

  @Override
  public void enterEveryRule(ParserRuleContext ctx) {

  }

  @Override
  public void exitEveryRule(ParserRuleContext ctx) {

  }
}
