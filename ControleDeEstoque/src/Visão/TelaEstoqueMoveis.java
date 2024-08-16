package Visão;

import java.util.ArrayList;
import java.util.List;

import javax.swing.JButton;
import javax.swing.JComboBox;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JTable;
import javax.swing.JTextArea;
import javax.swing.JTextField;
import javax.swing.RowFilter;
import javax.swing.event.ListSelectionEvent;
import javax.swing.event.ListSelectionListener;
import javax.swing.table.DefaultTableModel;
import javax.swing.table.TableModel;
import javax.swing.table.TableRowSorter;

import Controle.ControleEstoque;
import Modelo.Empresa;
import Modelo.Filial;
import Modelo.Movel;

import javax.swing.JScrollPane;

import java.awt.event.ActionEvent;
import java.awt.event.ActionListener;
import java.awt.event.KeyAdapter;
import java.awt.event.KeyEvent;
import java.awt.Font;

/**
 * O código apresenta uma classe chamada "TelaEstoqueMoveis" no pacote "Visão". 
 * A classe é responsável por criar uma interface gráfica para gerenciar o estoque 
 * de moveis em filiais de uma empresa.
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

public class TelaEstoqueMoveis {
	
	
	JFrame janela = new JFrame("Controle de Estoque");
	
	JLabel jlabelmoveis = new JLabel("Cadastro Filiais");
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
	

	    public TelaEstoqueMoveis() {
        
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

	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	        jlabelmoveis.setBounds(238, 11, 277, 38);
	        janela.getContentPane().add(jlabelmoveis);
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
	        
	        jtextempilhadeiras.setBounds(219, 204, 171, 20);
	    	jtextempilhadeiras.setColumns(10);
	    	jtextcaminhoes.setBounds(219, 179, 171, 20);
	    	jtextcaminhoes.setColumns(10);

	        
	        jbuttonexcluirfilialtabela.setBounds(613, 351, 89, 23);
	        janela.getContentPane().add(jbuttonexcluirfilialtabela);
	        janela.setResizable(false);
	        janela.setLocationRelativeTo(null);

	        modelo.addColumn("Filial");
	        modelo.addColumn("Endereco");
	        modelo.addColumn("Estado");
	        modelo.addColumn("Caminhoes Disponiveis");
	        modelo.addColumn("Empilhadeiras Disponiveis");
	        
	        
	        
	    }

	    private void configurarEventos() {
	    	
	    	List<Filial> listaFiliais = new ArrayList<>();
	    	
	    	DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	    	model.addRow(new Object[]{"CHS2", "Rua Antonio", "Utilizando", 5, 3});
	    	model.addRow(new Object[]{"ASW3", "Rua Antonio Alves", "Vazia", 2, 3});
	    	
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
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	DefaultTableModel model = (DefaultTableModel) tabela.getModel();
	            		    	model.addRow(new Object[]{"Mesa", "Marrom", "Tramontina", "Sala de Jantar", 2021, 1550, 1, 5, "Rustica"});
	            		    	model.addRow(new Object[]{"Cadeira", "Vermelha", "Ecomoveis", "Cadeira Piratine", 2022, 291, 2, 10, "Carvalho"});
	            		    	model.addRow(new Object[]{"Ping Pong", "Azul", "Klopf", "Mesa Dobravel", 2023, 1965, 3, 15, "MDF"});
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                if (tabela.getSelectedRow() == 2) {
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                if (tabela.getSelectedRow() == 4) {
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                if (tabela.getSelectedRow() == 5) {
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                if (tabela.getSelectedRow() == 6) {
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
	                if (tabela.getSelectedRow() == 7) {
	                            
	                            JFrame janelamoveis = new JFrame();
	                            
	                            JLabel jlabelmoveis = new JLabel("Cadastro Moveis");
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
	                        	JTextField jtexttipomadeira = new JTextField();
	                            JTextField jtextfiltro = new JTextField();
	                            
	                            JTextArea textareaempresa = new JTextArea();
	                            
	                        	JButton jbuttoncadastrar = new JButton("Cadastrar Produto:");
	                        	JButton jbuttonexcluir = new JButton("Excluir");
	                        	
	                        	JScrollPane scrollpaineltabela = new JScrollPane();
	                        	
	                        	DefaultTableModel modelo = new DefaultTableModel();
	                        	JTable tabela = new JTable(modelo);
	                        	
	                        	Empresa empresa = new Empresa("Empresa: CHSB\n", "Rua Padre Gama 251\n", 252891); 	                      	
	                        	
	                        	janelamoveis.setSize(920, 575);
	                            janelamoveis.getContentPane().setLayout(null);
	                            janelamoveis.setVisible(true);
	                        	
	                            
	                	        jlabelmoveis.setFont(new Font("Arial", Font.BOLD, 23));
	                	        jlabelmoveis.setBounds(296, 38, 248, 38);
	                	        janelamoveis.getContentPane().add(jlabelmoveis);

	                	        
	                	        jlabelnomeitem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelnomeitem.setBounds(10, 103, 150, 14);
	                	        janelamoveis.getContentPane().add(jlabelnomeitem);

	                	        
	                	        jlabelcor.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcor.setBounds(10, 128, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelcor);

	                	        
	                	        jlabelmarca.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmarca.setBounds(10, 153, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmarca);

	                	        
	                	        jlabelmodelo.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelmodelo.setBounds(10, 178, 119, 14);
	                	        janelamoveis.getContentPane().add(jlabelmodelo);

	                	        
	                	        jlabeldatadefabricacao.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabeldatadefabricacao.setBounds(10, 203, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabeldatadefabricacao);

	                	        
	                	        jlabelvalorproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvalorproduto.setBounds(10, 228, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvalorproduto);

	                	        
	                	        jlabelcodigodoproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelcodigodoproduto.setBounds(10, 255, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelcodigodoproduto);

	                	        
	                	        jlabelquantidadeproduto.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelquantidadeproduto.setBounds(10, 283, 171, 14);
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);

	                	        
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
	                	        janelamoveis.getContentPane().add(jtextfiltro);
	                	        jtextfiltro.setColumns(10);
	                	        
	                	        
	                	        jlabelvoltagem.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelvoltagem.setBounds(10, 308, 138, 14);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jtexttipomadeira);
	                	        jtexttipomadeira.setBounds(144, 305, 171, 20);
	                	        jtexttipomadeira.setColumns(10);
	                	        
	                	        
	                	        jlabelfiltro.setFont(new Font("Arial", Font.BOLD, 13));
	                	        jlabelfiltro.setBounds(349, 354, 73, 14);
	                	        janelamoveis.getContentPane().add(jlabelfiltro);
	                            
	                	        
	                	        jbuttoncadastrar.setBounds(39, 350, 147, 23);
	                	        janelamoveis.getContentPane().add(jbuttoncadastrar);
	                	        
	                	        
	                	        janelamoveis.getContentPane().add(jlabelquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jtextnomeitem);
	                	        janelamoveis.getContentPane().add(jtextcor);
	                	        janelamoveis.getContentPane().add(jtextmarca);
	                	        janelamoveis.getContentPane().add(jtextmodelo);
	                	        janelamoveis.getContentPane().add(jtextdatafabricacao);
	                	        janelamoveis.getContentPane().add(jtextvalorproduto);
	                	        janelamoveis.getContentPane().add(jtextcodigoproduto);
	                	        janelamoveis.getContentPane().add(jtextquantidadeproduto);
	                	        janelamoveis.getContentPane().add(jlabelvoltagem);
	                	        
	                	        
	                	        scrollpaineltabela.setBounds(0, 384, 904, 152);
	                	        janelamoveis.getContentPane().add(scrollpaineltabela);	        
	                	        scrollpaineltabela.setViewportView(tabela);
	                	        
	                	        textareaempresa.setText(empresa.getNome() + empresa.getEndereço() + "CNPJ: " + empresa.getCnpj());
	                	        textareaempresa.setEditable(false);
	                	        textareaempresa.setFont(new Font("Arial", Font.BOLD, 13));
	                	        textareaempresa.setBounds(723, 308, 171, 68);
	                	        janelamoveis.getContentPane().add(textareaempresa);
	                	        
	                	        jbuttonexcluir.setBounds(208, 350, 89, 23);
	                	        janelamoveis.getContentPane().add(jbuttonexcluir);
	                	        
	                	        
	                	        modelo.addColumn("Nome");
	                	        modelo.addColumn("Cor");
	                	        modelo.addColumn("Marca");
	                	        modelo.addColumn("Modelo");

	                	        modelo.addColumn("Data Fabricação");
	                	        modelo.addColumn("Valor do Produto");
	                	        modelo.addColumn("Codigo do Produto");
	                	        modelo.addColumn("Quantidade Produto");
	                	        modelo.addColumn("Tipo Madeira");
	                	        
	                	        Movel movel = new Movel();
	                	        
	            		    	ControleEstoque controleestoque = new ControleEstoque();
	            		    	
	            		    	jbuttoncadastrar.addActionListener(new ActionListener() {
	            		            public void actionPerformed(ActionEvent e) {

	            		                movel.setNomeitem(jtextnomeitem.getText());
	            		                movel.setCor(jtextcor.getText());
	            		                movel.setMarca(jtextmarca.getText());
	            		                movel.setModelo(jtextmodelo.getText());
	            		                
	            		                movel.setDatadefabricação(Double.parseDouble(jtextdatafabricacao.getText()));
	            		                movel.setValorproduto(Double.parseDouble(jtextvalorproduto.getText()));
	            		                movel.setCodigoproduto(Double.parseDouble(jtextcodigoproduto.getText()));
	            		                movel.setQuantidadeproduto(Double.parseDouble(jtextquantidadeproduto.getText()));
	            		                movel.setTipodemadeira(jtexttipomadeira.getText());
	            		                	               	     	                

	            		                if (controleestoque.salvarmoveis(movel)) {
	            		                    if (movel.getFilial() != null) {
	            		                        modelo.addRow(new Object[]{
	            		                        	
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
	            		                        });
	            		                    } else {
	            		                        modelo.addRow(new Object[]{
	            		                            
	            		                            movel.getNomeitem(),
	            		                            movel.getCor(),
	            		                            movel.getMarca(),
	            		                            movel.getModelo(),
	            		                            movel.getDatadefabricação(),
	            		                            movel.getValorproduto(),
	            		                            movel.getCodigoproduto(),
	            		                            movel.getQuantidadeproduto(),
	            		                            movel.getTipodemadeira()
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
	            		                    jtexttipomadeira.setText("");
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
			
			
		}
	}