package Visão;

import javax.swing.JFrame;
import javax.swing.JLabel;
import java.awt.Font;
import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.util.ArrayList;
import java.util.List;

import javax.swing.JTextField;
import javax.swing.RowFilter;
import javax.swing.event.ListSelectionEvent;
import javax.swing.event.ListSelectionListener;
import javax.swing.table.DefaultTableModel;
import javax.swing.table.TableModel;
import javax.swing.table.TableRowSorter;
import javax.swing.JComboBox;

import Modelo.Filial;
import Modelo.Eletronico;
import Modelo.Empresa;
import Controle.ControleEstoque;

import javax.swing.JButton;
import javax.swing.JTable;
import javax.swing.JTextArea;
import javax.swing.JScrollPane;

/**
 * O código apresenta uma classe chamada "TelaEstoqueEletronico" no pacote "Visão". 
 * A classe é responsável por criar uma interface gráfica para gerenciar o estoque 
 * de eletrônicos em filiais de uma empresa.
 * 
 * A classe possui os seguintes componentes visuais:
 * Uma janela principal, representada por um objeto JFrame.
 * Uma janela principal, representada por um objeto JFrame.
 * Vários campos de texto (JTextField) para inserir dados.
 * Vários botões (JButton) para realizar ações na interface.
 * Uma tabela (JTable) para exibir os dados das filiais cadastradas.
 * 
 * @author Carlos Henrique
 * @author André João
 * @author Sunamita Vitória
 * 
 * @version 2.0
 */

public class TelaEstoqueEletronico {
	
	
	JFrame janela = new JFrame("Controle de Estoque");
	
	JLabel jlabeleletronicos = new JLabel("Cadastro Filiais");
	JLabel jlabelfilial = new JLabel("Filial:");
	JLabel jlabelfiliaiscadastrada = new JLabel("Filiais Cadastrada:");
	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	JLabel jlabelendereco = new JLabel("Endereço:\r\n");
	JLabel Estado = new JLabel("Estado:");
	JLabel jlabelcaminhoes = new JLabel("Caminhoes Disponiveis:");
	JLabel jlabelempilhadeiras = new JLabel("Empilhadeiras Disponiveis:");
	
	JTextField jtextfilial = new JTextField();
	JTextField jtextfiltro = new JTextField();
	JTextField jtextendereco = new JTextField();
	JTextField jtextestado = new JTextField();
	JTextField jtextcaminhoes = new JTextField();
	JTextField jtextempilhadeiras = new JTextField();
	
	JButton jbuttoncadastrar = new JButton("Inserir Filial");
	JButton jbuttoncadastrarfilial = new JButton("Cadastrar Filial");
	JButton jbuttonexcluirfilial = new JButton("Excluir");
	JButton jbuttonexcluirfilialtabela = new JButton("Deletar");
	
	JComboBox<Filial> comboboxfilial  = new JComboBox<Filial>();
	
	DefaultTableModel modelo = new DefaultTableModel();
	JTable tabela = new JTable(modelo);
	
	JScrollPane scrollpaineltabela = new JScrollPane(tabela);
	
	Filial filial = new Filial();
	
	ArrayList<Filial> filiais = new ArrayList<Filial>();
	
	

	    public TelaEstoqueEletronico() {	    	
	        
	        configurarInterface();
	        configurarEventos();
	        configurartabelavazia();
	        configurarTabelaCadastrada();
	        configurarTabelaCadastrada1();
	        configurarTabelaCadastrada2();
	        configurarTabelaCadastrada3();
	        configurarTabelaCadastrada4();
	        configurarTabelaCadastrada5();
	    }

	    private void configurarInterface() {
	    		                   
	        modelo = new DefaultTableModel();
	        tabela = new JTable(modelo);
	        scrollpaineltabela = new JScrollPane(tabela);	        
	        
	        jtextfilial.setBounds(219, 100, 171, 20);
	        jtextfilial.setColumns(10);

	        janela.setSize(920, 575);
	        janela.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
	        janela.getContentPane().setLayout(null);
	        janela.setVisible(true);

	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        jlabeleletronicos.setBounds(238, 11, 277, 38);
	        janela.getContentPane().add(jlabeleletronicos);
	        jlabelfilial.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelfilial.setBounds(39, 103, 57, 14);

	        janela.getContentPane().add(jlabelfilial);

	        janela.getContentPane().add(jtextfilial);
	        comboboxfilial.setBounds(158, 259, 617, 22);

	        janela.getContentPane().add(comboboxfilial);
	        jlabelfiliaiscadastrada.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelfiliaiscadastrada.setBounds(28, 263, 130, 14);

	        janela.getContentPane().add(jlabelfiliaiscadastrada);
	        
	        jbuttoncadastrar.setBounds(308, 351, 147, 23);

	        janela.getContentPane().add(jbuttoncadastrar);
	        jbuttoncadastrarfilial.setBounds(465, 351, 138, 23);

	        janela.getContentPane().add(jbuttoncadastrarfilial);
	        scrollpaineltabela.setBounds(0, 385, 904, 151);

	        janela.getContentPane().add(scrollpaineltabela);
	        scrollpaineltabela.setViewportView(tabela);
	        
	        
	        jbuttonexcluirfilial.setBounds(785, 259, 89, 23);
	        janela.getContentPane().add(jbuttonexcluirfilial);
	        
	        
	        jtextfiltro.setBounds(111, 354, 171, 20);
	        janela.getContentPane().add(jtextfiltro);
	        jtextfiltro.setColumns(10);
	        
	       
	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelfiltro.setBounds(20, 360, 123, 14);
	        janela.getContentPane().add(jlabelfiltro);
	        
	        
	        jlabelendereco.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelendereco.setBounds(39, 128, 104, 14);
	        janela.getContentPane().add(jlabelendereco);
	        
	        
	        Estado.setFont(new Font("Arial", Font.BOLD, 13));
	        Estado.setBounds(39, 157, 89, 14);
	        janela.getContentPane().add(Estado);
	        
	        
	        jtextendereco.setBounds(219, 125, 171, 20);
	        janela.getContentPane().add(jtextendereco);
	        jtextendereco.setColumns(10);
	        
	        
	        jtextestado.setBounds(219, 154, 171, 20);
	        janela.getContentPane().add(jtextestado);
	        jtextestado.setColumns(10);
	        jlabelcaminhoes.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelcaminhoes.setBounds(39, 182, 171, 14);
	        
	        janela.getContentPane().add(jlabelcaminhoes);
	        jlabelempilhadeiras.setFont(new Font("Arial", Font.BOLD, 13));
	        jlabelempilhadeiras.setBounds(39, 207, 182, 14);
	        
	        janela.getContentPane().add(jlabelempilhadeiras);
	        
	        janela.getContentPane().add(jtextcaminhoes);
	        
	        janela.getContentPane().add(jtextempilhadeiras);
	        
	        
	        
	        jbuttonexcluirfilialtabela.setBounds(613, 351, 89, 23);
	        janela.getContentPane().add(jbuttonexcluirfilialtabela);
	        janela.setResizable(false);
	        janela.setLocationRelativeTo(null);
	        
	        jtextempilhadeiras.setBounds(219, 204, 171, 20);
	    	jtextempilhadeiras.setColumns(10);
	    	jtextcaminhoes.setBounds(219, 179, 171, 20);
	    	jtextcaminhoes.setColumns(10);
	    	

	        modelo.addColumn("Filial");
	        modelo.addColumn("Endereco");
	        modelo.addColumn("Estado");
	        modelo.addColumn("Caminhoes Disponiveis");
	        modelo.addColumn("Empilhadeiras Disponiveis");
	        
	        
	        
	    }

	    private void configurarEventos() {
	    	
	    	List<Filial> listaFiliais = new ArrayList<>();
	    	
	    	DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	    	model.addRow(new Object[]{"CHSB1", "Rua Antonio", "Utilizando", 5, 3});
	    	model.addRow(new Object[]{"EWS3", "Rua Antonio Alves", "Vazia", 2, 3});
	    	
	    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            public void actionPerformed(ActionEvent e) {

	            	Filial filialSelecionada = (Filial) comboboxfilial.getSelectedItem();

	                // Verifique se uma filial foi selecionada
	            
	                if (filialSelecionada != null) {
	                    // Obtenha os dados da filial
	                	
	                    String nome = filialSelecionada.getNome();
	                    String endereco = filialSelecionada.getEndereço();
	                    String estado = filialSelecionada.getEstado();
	                    int caminhoes = filialSelecionada.getCaminhaoDentro();
	                    int empilhadeiras = filialSelecionada.getEmpilhadeiradisponivel();


	                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	                    model.addRow(new Object[]{nome, endereco, estado, caminhoes, empilhadeiras});
	                    
	                    
	                }                   
	            }
	        });
	    	
	    	
	    	
	    	
	    	
	    	jbuttoncadastrarfilial.addActionListener(new ActionListener() {
	    	    public void actionPerformed(ActionEvent e) {
	    	        Filial filial = new Filial(); 

	    	        filial.setNome(jtextfilial.getText());
	    	        filial.setEndereço(jtextendereco.getText());
	    	        filial.setEstado(jtextestado.getText());
	    	        filial.setCaminhaoDentro(Integer.parseInt(jtextcaminhoes.getText()));
	    	        filial.setEmpilhadeiradisponivel(Integer.parseInt(jtextempilhadeiras.getText()));
	    	        
	    	        listaFiliais.add(filial);
	    	        comboboxfilial.addItem(filial);

	    	        jtextfilial.setText("");
	    	        jtextendereco.setText("");
	    	        jtextestado.setText("");
	    	        jtextcaminhoes.setText("");
	    	        jtextempilhadeiras.setText("");
	    	    }
	    	});
	        
	        jbuttonexcluirfilial.addActionListener(new ActionListener() {
	        	
	        	public void actionPerformed(ActionEvent e) {
	        		
	        		int selectedIndex = comboboxfilial.getSelectedIndex();
	                
	                if (selectedIndex != -1) {
	                    comboboxfilial.removeItemAt(selectedIndex);
	                    listaFiliais.remove(selectedIndex);
	                                    
	                }	
	        	}
	        });
	        
	        jbuttonexcluirfilialtabela.addActionListener(new ActionListener() {
	        	
	        	public void actionPerformed(ActionEvent e) {
	        		
	        		int selectedRow = tabela.getSelectedRow();
	                
	                if (selectedRow != -1) {
	                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	                    model.removeRow(selectedRow);
	                    
	                }
	        	}
	        });
	        
	        jtextfiltro.addKeyListener(new KeyAdapter() {
				
				public void keyPressed(KeyEvent e) {
					
					DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
					final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
					tabela.setRowSorter(sorter);
					
					String txt = jtextfiltro.getText();
			        if(txt.length() == 0)
			        {
			            sorter.setRowFilter(null);
			        }
			        else
			        {
			            sorter.setRowFilter(RowFilter.regexFilter(txt));
			        }
					
				}
			});
	        
	        tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 0) {
	                    
	                    
	                    JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	    		    	model.addRow(new Object[]{"Notebook", "Preto", "Samsumg", "I5 10050H", 2022, 2500, 1, 5, 220});
	    		    	model.addRow(new Object[]{"PC", "Rosa", "Dell", "I7 9500K", 2023, 5000, 2, 10, 220});
	    		    	model.addRow(new Object[]{"MacBook", "Branco", "Apple", "I9 11050", 2024, 25000, 3, 15, 220});
	    		    	model.addRow(new Object[]{"Tablet", "Azul", "Positivo", "Tab Q10", 2021, 900, 4, 20, 110});
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	        
	    }
	    
	    private void configurartabelavazia() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 1) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	        
	    }
	    	
	    private void configurarTabelaCadastrada() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 2 ) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	        
	    }
	    
	    private void configurarTabelaCadastrada1() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 3) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	    	
	    }
	    
	    private void configurarTabelaCadastrada2() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 4 ) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	    	
	    }
	    
	    private void configurarTabelaCadastrada3() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 5 ) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	    	
	    }
	    
	    private void configurarTabelaCadastrada4() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 6 ) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	    	
	    }
	    private void configurarTabelaCadastrada5() {
	    	
	    	tabela.getSelectionModel().addListSelectionListener(new ListSelectionListener() {
	            @Override
	            public void valueChanged(ListSelectionEvent e) {
	            	
	                // Verificar se a linha 0 está selecionada
	                if (tabela.getSelectedRow() == 7 ) {
	                	
	                	JFrame janelaeletronicos = new JFrame();
	                    
	                    JLabel jlabeleletronicos = new JLabel("Cadastro Eletronicos");
	                	JLabel jlabelnomeitem = new JLabel("Nome do Produto: ");
	                	JLabel jlabelcor = new JLabel("Cor do Produto: ");
	                	JLabel jlabelmarca = new JLabel("Marca: ");
	                	JLabel jlabelmodelo = new JLabel("Modelo:");
	                	JLabel jlabeldatadefabricacao = new JLabel("Data Fabricação:");
	                	JLabel jlabelvalorproduto = new JLabel("Valor do Produto:");
	                	JLabel jlabelcodigodoproduto = new JLabel("Codigo do Produto:");
	                	JLabel jlabelquantidadeproduto = new JLabel("Quantidade Produto:");
	                	JLabel jlabelvoltagem = new JLabel("Voltagem:");
	                	JLabel jlabelfiltro = new JLabel("Pesquisa:");
	                	
	                	JTextField jtextnomeitem = new JTextField();
	                	JTextField jtextcor = new JTextField();
	                	JTextField jtextmarca = new JTextField();
	                	JTextField jtextmodelo = new JTextField();
	                	JTextField jtextdatafabricacao = new JTextField();
	                	JTextField jtextvalorproduto = new JTextField();
	                	JTextField jtextcodigoproduto = new JTextField();
	                	JTextField jtextquantidadeproduto = new JTextField();
	                	JTextField jtextvoltagem = new JTextField();
	                    JTextField jtextfiltro = new JTextField();
	                    
	                    JTextArea textareaempresa = new JTextArea();
	                    
	                	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                	JButton jbuttonexcluir = new JButton("Excluir");
	                	
	                	JScrollPane scrollpaineltabela = new JScrollPane();
	                	
	                	DefaultTableModel modelo = new DefaultTableModel();
	                	JTable tabela = new JTable(modelo);
	                	
	                	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                	
	                	janelaeletronicos.setSize(920, 575);
	                    janelaeletronicos.getContentPane().setLayout(null);
	                    janelaeletronicos.setVisible(true);
	                	
	                    
	        	        jlabeleletronicos.setFont(new Font("Arial", Font.BOLD, 23));
	        	        jlabeleletronicos.setBounds(296, 38, 248, 38);
	        	        janelaeletronicos.getContentPane().add(jlabeleletronicos);

	        	        
	        	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelnomeitem);

	        	        
	        	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcor.setBounds(10, 128, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcor);

	        	        
	        	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmarca.setBounds(10, 153, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmarca);

	        	        
	        	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelmodelo.setBounds(10, 178, 119, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelmodelo);

	        	        
	        	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabeldatadefabricacao);

	        	        
	        	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvalorproduto);

	        	        
	        	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelcodigodoproduto);

	        	        
	        	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);

	        	        
	        	        jtextquantidadeproduto.setBounds(144, 280, 171, 20);
	        	        jtextcodigoproduto.setBounds(144, 252, 171, 20);
	        	        jtextvalorproduto.setBounds(144, 228, 171, 20);
	        	        jtextdatafabricacao.setBounds(144, 200, 171, 20);
	        	        jtextmodelo.setBounds(144, 175, 171, 20);
	        	        jtextmarca.setBounds(144, 150, 171, 20);
	        	        jtextcor.setBounds(144, 125, 171, 20);
	        	        jtextnomeitem.setBounds(144, 100, 171, 20);

	        	                        	        
	        	        jbuttoncadastrar.setBounds(357, 346, 147, 23);
	        	        
	        	        
	        	        jtextfiltro.setBounds(432, 351, 171, 20);
	        	        janelaeletronicos.getContentPane().add(jtextfiltro);
	        	        jtextfiltro.setColumns(10);
	        	        
	        	        
	        	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jtextvoltagem);
	        	        jtextvoltagem.setBounds(144, 305, 171, 20);
	        	        jtextvoltagem.setColumns(10);
	        	        
	        	        
	        	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	        	        jlabelfiltro.setBounds(349, 354, 73, 14);
	        	        janelaeletronicos.getContentPane().add(jlabelfiltro);
	                    
	        	        
	        	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttoncadastrar);
	        	        
	        	        
	        	        janelaeletronicos.getContentPane().add(jlabelquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jtextnomeitem);
	        	        janelaeletronicos.getContentPane().add(jtextcor);
	        	        janelaeletronicos.getContentPane().add(jtextmarca);
	        	        janelaeletronicos.getContentPane().add(jtextmodelo);
	        	        janelaeletronicos.getContentPane().add(jtextdatafabricacao);
	        	        janelaeletronicos.getContentPane().add(jtextvalorproduto);
	        	        janelaeletronicos.getContentPane().add(jtextcodigoproduto);
	        	        janelaeletronicos.getContentPane().add(jtextquantidadeproduto);
	        	        janelaeletronicos.getContentPane().add(jlabelvoltagem);
	        	        
	        	        
	        	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	        	        janelaeletronicos.getContentPane().add(scrollpaineltabela);	        
	        	        scrollpaineltabela.setViewportView(tabela);
	        	        
	        	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	        	        textareaempresa.setEditable(false);
	        	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	        	        textareaempresa.setBounds(723, 308, 171, 68);
	        	        janelaeletronicos.getContentPane().add(textareaempresa);
	        	        
	        	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	        	        janelaeletronicos.getContentPane().add(jbuttonexcluir);
	        	        
	        	        
	        	        modelo.addColumn("Nome");
	        	        modelo.addColumn("Cor");
	        	        modelo.addColumn("Marca");
	        	        modelo.addColumn("Modelo");

	        	        modelo.addColumn("Data Fabricação");
	        	        modelo.addColumn("Valor do Produto");
	        	        modelo.addColumn("Codigo do Produto");
	        	        modelo.addColumn("Quantidade Produto");
	        	        modelo.addColumn("Voltagem");
	        	        
	        	        Eletronico eletronico = new Eletronico();
	        	        
	    		    	ControleEstoque controleestoque = new ControleEstoque();
	    		   
	    		    	
	    		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	    		            public void actionPerformed(ActionEvent e) {

	    		                eletronico.setNomeitem(jtextnomeitem.getText());
	    		                eletronico.setCor(jtextcor.getText());
	    		                eletronico.setMarca(jtextmarca.getText());
	    		                eletronico.setModelo(jtextmodelo.getText());
	    		                
	    		                eletronico.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	    		                eletronico.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	    		                eletronico.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	    		                eletronico.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	    		                eletronico.setVoltagem(Double.parseDouble(jtextvoltagem.getText()));
	    		                	               	     	                

	    		                if (controleestoque.salvareletronicos(eletronico)) {
	    		                    if (eletronico.getFilial() != null) {
	    		                        modelo.addRow(new Object[]{
	    		                        	
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    } else {
	    		                        modelo.addRow(new Object[]{
	    		                            
	    		                            eletronico.getNomeitem(),
	    		                            eletronico.getCor(),
	    		                            eletronico.getMarca(),
	    		                            eletronico.getModelo(),
	    		                            eletronico.getDatadefabricação(),
	    		                            eletronico.getValorproduto(),
	    		                            eletronico.getCodigoproduto(),
	    		                            eletronico.getQuantidadeproduto(),
	    		                            eletronico.getVoltagem()
	    		                        });
	    		                    }

	    		                    jtextnomeitem.setText("");
	    		                    jtextcor.setText("");
	    		                    jtextmarca.setText("");
	    		                    jtextmodelo.setText("");
	    		                    jtextdatafabricacao.setText("");
	    		                    jtextvalorproduto.setText("");
	    		                    jtextcodigoproduto.setText("");
	    		                    jtextquantidadeproduto.setText("");
	    		                    jtextvoltagem.setText("");
	    		                }

	    		                
	    		            }
	    		            
	    		        });
	    		    	
	    		    	jtextfiltro.addKeyListener(new KeyAdapter() {
        					
        					public void keyPressed(KeyEvent e) {
        						
        						DefaultTableModel filtro = (DefaultTableModel) tabela.getModel();
        						final TableRowSorter<TableModel> sorter = new TableRowSorter<>(filtro);
        						tabela.setRowSorter(sorter);
        						
        						String txt = jtextfiltro.getText();
        				        if(txt.length() == 0)
        				        {
        				            sorter.setRowFilter(null);
        				        }
        				        else
        				        {
        				            sorter.setRowFilter(RowFilter.regexFilter(txt));
        				        }
        						
        					}
        				});
        		    	
        		    	jbuttonexcluir.addActionListener(new ActionListener() {
        		    		
        		        	public void actionPerformed(ActionEvent e) {
        		        		
        		        		int selectedRow = tabela.getSelectedRow();
        		                
        		                if (selectedRow != -1) {
        		                    DefaultTableModel model = (DefaultTableModel) tabela.getModel();
        		                    model.removeRow(selectedRow);
        		                }
        		            }
        		        });
	    		    	
	    		    	
	                    
	                }
	                
	            }
	            
	        });
	
	    }
	    	
	    

	    public void setVisible(boolean b) {
	        // Implementação do método setVisible
	    }
	}