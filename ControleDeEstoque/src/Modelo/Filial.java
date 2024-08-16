
package Modelo;

import java.util.ArrayList;
import java.util.List;

/**
 * A classe Filial representa uma filial de uma empresa. Ela herda as características da classe Estoque
 * e adiciona informações específicas de uma filial, como nome, endereço e estado.
 * 
 * @author Carlos Henrique
 * @author André João
 * @author Sunamita Vitória
 * 
 * @version 2.0
 */

public class Filial extends Estoque{
	
	
	private String nome;
	private String endereço;
	private String estado;
	
	List<Filial> listaFiliais = new ArrayList<>();
	
	/**
     * Construtor da classe Filial que recebe o nome, endereço e estado da filial.
     *
     * @param nome     O nome da filial.
     * @param endereço O endereço da filial.
     * @param estado   O estado onde a filial está localizada.
     */
		

	public Filial(String nome, String endereço, String estado) {
		super();
		this.nome = nome;
		this.endereço = endereço;
		this.estado = estado;
	}

	/**
     * Construtor vazio da classe Filial.
     */
	
	public Filial() {
		
	}
	
	/**
     * Retorna a lista de filiais.
     *
     * @return A lista de filiais.
     */

	public List<Filial> getListaFiliais() {
		return listaFiliais;
	}
	
	/**
     * Define a lista de filiais.
     *
     * @param listaFiliais A lista de filiais.
     */

	public void setListaFiliais(List<Filial> listaFiliais) {
		this.listaFiliais = listaFiliais;
	}
	
	/**
     * Retorna o nome da filial.
     *
     * @return O nome da filial.
     */
	
	public String getNome() {
		return nome;
	}
	
	/**
     * Define o nome da filial.
     *
     * @param nome O nome da filial.
     */
	
	public void setNome(String nome) {
		this.nome = nome;
	}
	
	/**
     * Retorna o endereço da filial.
     *
     * @return O endereço da filial.
     */
	
	public String getEndereço() {
		return endereço;
	}
	
	/**
     * Define o endereço da filial.
     *
     * @param endereço O endereço da filial.
     */
	
	public void setEndereço(String endereço) {
		this.endereço = endereço;
	}
	
	/**
     * Define o endereço da filial.
     *
     * @param endereço O endereço da filial.
     */
	
	public String getEstado() {
		return estado;
	}
	
	/**
     * Define o estado onde a filial está localizada.
     *
     * @param estado O estado da filial.
     */
	
	public void setEstado(String estado) {
		this.estado = estado;
	}

	@Override
	public String toString() {
	    return "Nome:" + getNome() + ", Endereço: " + getEndereço() + ", Estado: " + getEstado() + 
	    		", Caminhoes: " + getCaminhaoDentro() + ", Empilhadeiras: " +getEmpilhadeiradisponivel();
	}

	
	
	
	
	
	
	
}
